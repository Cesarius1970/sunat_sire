// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Cliente Asíncrono del Protocolo TUS 1.0.0 (`sire_cliente_tus`)
//!
//! Implementa las operaciones del protocolo TUS (POST creación, HEAD consulta de offset,
//! y PATCH transmisión de fragmentos) con inyección de autenticación Bearer y control
//! de reintentos sobre Tokio y Reqwest.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE, LOCATION};
use tracing::{debug, error, info, instrument, warn};

use crate::sire_errores::{SireError, SireResultado};
use crate::sire_tus::sire_configuracion_tus::SireConfiguracionTus;
use crate::sire_tus::sire_metadatos_tus::SireMetadatosTus;
use crate::sire_tus::sire_progreso_tus::SireProgresoTus;

/// Cabeceras estándar de la especificación TUS 1.0.0.
const CABECERA_TUS_RESUMABLE: &str = "Tus-Resumable";
const CABECERA_UPLOAD_LENGTH: &str = "Upload-Length";
const CABECERA_UPLOAD_OFFSET: &str = "Upload-Offset";
const CABECERA_UPLOAD_METADATA: &str = "Upload-Metadata";
const TUS_VERSION: &str = "1.0.0";
const CONTENT_TYPE_OFFSET_STREAM: &str = "application/offset+octet-stream";

/// Resultado de una carga masiva completada mediante el protocolo TUS.
#[derive(Debug, Clone)]
pub struct SireRespuestaTus {
    /// URL asignada al recurso de subida por el servidor de SUNAT.
    pub url_upload: String,
    /// Total de bytes transferidos exitosamente.
    pub bytes_subidos: u64,
    /// Número de ticket de proceso emitido por SUNAT, si fue retornado de forma directa.
    pub num_ticket: Option<String>,
}

/// Cliente para la ejecución de cargas resumibles TUS 1.0.0 hacia SUNAT SIRE.
#[derive(Debug, Clone)]
pub struct SireClienteTus {
    cliente_http: reqwest::Client,
    configuracion: SireConfiguracionTus,
}

impl SireClienteTus {
    /// Inicializa un nuevo cliente TUS con la configuración y cliente HTTP indicados.
    pub fn nuevo(cliente_http: reqwest::Client, configuracion: SireConfiguracionTus) -> Self {
        Self {
            cliente_http,
            configuracion,
        }
    }

    /// Retorna una referencia a la configuración del cliente TUS.
    pub fn configuracion(&self) -> &SireConfiguracionTus {
        &self.configuracion
    }

    /// Fase 1: Inicia una nueva sesión de carga TUS mediante una petición `POST`.
    ///
    /// Envía `Tus-Resumable`, `Upload-Length` y `Upload-Metadata`, retornando la URL
    /// de subida del recurso indicada en la cabecera `Location`.
    #[instrument(skip(self, token_bearer, metadatos), level = "info")]
    pub async fn crear_sesion_upload(
        &self,
        endpoint_creacion: &str,
        token_bearer: &str,
        longitud_total: u64,
        metadatos: &SireMetadatosTus,
    ) -> SireResultado<String> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token_bearer))
                .map_err(|e| SireError::Autenticacion(e.to_string()))?,
        );
        headers.insert(
            HeaderName::from_static(CABECERA_TUS_RESUMABLE),
            HeaderValue::from_static(TUS_VERSION),
        );
        headers.insert(
            HeaderName::from_static(CABECERA_UPLOAD_LENGTH),
            HeaderValue::from_str(&longitud_total.to_string())
                .map_err(|e| SireError::Tus(format!("Error en Upload-Length header: {}", e)))?,
        );
        headers.insert(
            HeaderName::from_static(CABECERA_UPLOAD_METADATA),
            HeaderValue::from_str(&metadatos.a_cabecera_upload_metadata())
                .map_err(|e| SireError::Tus(format!("Error en Upload-Metadata header: {}", e)))?,
        );

        debug!(
            endpoint = %endpoint_creacion,
            longitud = %longitud_total,
            "Iniciando sesión TUS POST"
        );

        let respuesta = self
            .cliente_http
            .post(endpoint_creacion)
            .headers(headers)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        // TUS 1.0.0 especifica 201 Created; algunos servidores SUNAT devuelven 200 OK.
        if !estado.is_success() {
            let cuerpo = respuesta.text().await.unwrap_or_default();
            error!(estado = %estado, cuerpo = %cuerpo, "Fallo al crear sesión de carga TUS");
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en POST TUS creación: {}", cuerpo),
            });
        }

        let cabecera_location = respuesta
            .headers()
            .get(LOCATION)
            .ok_or_else(|| {
                SireError::Tus("El servidor no retornó la cabecera 'Location' obligatoria".to_string())
            })?
            .to_str()
            .map_err(|e| SireError::Tus(format!("Cabecera Location inválida: {}", e)))?;

        // Resolver URL absoluta si la respuesta fue relativa
        let url_upload = if cabecera_location.starts_with("http://")
            || cabecera_location.starts_with("https://")
        {
            cabecera_location.to_string()
        } else {
            let url_base = reqwest::Url::parse(endpoint_creacion)
                .map_err(|e| SireError::Tus(format!("URL base inválida: {}", e)))?;
            url_base
                .join(cabecera_location)
                .map_err(|e| SireError::Tus(format!("Error al unir Location relativa: {}", e)))?
                .to_string()
        };

        info!(url = %url_upload, "Sesión de carga TUS creada exitosamente");
        Ok(url_upload)
    }

    /// Fase 2: Consulta el desplazamiento (*offset*) actual del archivo mediante una petición `HEAD`.
    ///
    /// Permite conocer cuántos bytes ya fueron transferidos al servidor para reanudar la carga.
    #[instrument(skip(self, token_bearer), level = "debug")]
    pub async fn consultar_offset(&self, url_upload: &str, token_bearer: &str) -> SireResultado<u64> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token_bearer))
                .map_err(|e| SireError::Autenticacion(e.to_string()))?,
        );
        headers.insert(
            HeaderName::from_static(CABECERA_TUS_RESUMABLE),
            HeaderValue::from_static(TUS_VERSION),
        );

        let respuesta = self
            .cliente_http
            .head(url_upload)
            .headers(headers)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        if !estado.is_success() {
            let cuerpo = respuesta.text().await.unwrap_or_default();
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en HEAD TUS consulta de offset: {}", cuerpo),
            });
        }

        let valor_offset = respuesta
            .headers()
            .get(CABECERA_UPLOAD_OFFSET)
            .ok_or_else(|| {
                SireError::Tus("El servidor no retornó la cabecera 'Upload-Offset' en HEAD".to_string())
            })?
            .to_str()
            .map_err(|e| SireError::Tus(format!("Cabecera Upload-Offset no es UTF-8: {}", e)))?
            .parse::<u64>()
            .map_err(|e| SireError::Tus(format!("Upload-Offset no es un entero numérico: {}", e)))?;

        debug!(url = %url_upload, offset = %valor_offset, "Offset TUS consultado exitosamente");
        Ok(valor_offset)
    }

    /// Fase 3: Envía un fragmento del archivo mediante una petición `PATCH`.
    ///
    /// Retorna el nuevo `Upload-Offset` reportado por el servidor y un posible número de ticket.
    #[instrument(skip(self, token_bearer, datos_chunk), level = "debug")]
    pub async fn enviar_chunk(
        &self,
        url_upload: &str,
        token_bearer: &str,
        offset_actual: u64,
        datos_chunk: &[u8],
    ) -> SireResultado<(u64, Option<String>)> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token_bearer))
                .map_err(|e| SireError::Autenticacion(e.to_string()))?,
        );
        headers.insert(
            HeaderName::from_static(CABECERA_TUS_RESUMABLE),
            HeaderValue::from_static(TUS_VERSION),
        );
        headers.insert(
            HeaderName::from_static(CABECERA_UPLOAD_OFFSET),
            HeaderValue::from_str(&offset_actual.to_string())
                .map_err(|e| SireError::Tus(format!("Upload-Offset inválido: {}", e)))?,
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static(CONTENT_TYPE_OFFSET_STREAM),
        );

        let respuesta = self
            .cliente_http
            .patch(url_upload)
            .headers(headers)
            .body(datos_chunk.to_vec())
            .timeout(self.configuracion.timeout_chunk)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        // TUS especifica 204 No Content para PATCH exitoso, o 200 OK
        if !estado.is_success() {
            let cuerpo = respuesta.text().await.unwrap_or_default();
            error!(
                estado = %estado,
                offset = %offset_actual,
                cuerpo = %cuerpo,
                "Fallo en PATCH TUS al transmitir fragmento"
            );
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en PATCH TUS chunk en offset {}: {}", offset_actual, cuerpo),
            });
        }

        let nuevo_offset = respuesta
            .headers()
            .get(CABECERA_UPLOAD_OFFSET)
            .ok_or_else(|| {
                SireError::Tus("El servidor no retornó cabecera 'Upload-Offset' en respuesta PATCH".to_string())
            })?
            .to_str()
            .map_err(|e| SireError::Tus(format!("Upload-Offset no es UTF-8: {}", e)))?
            .parse::<u64>()
            .map_err(|e| SireError::Tus(format!("Upload-Offset devuelto no es numérico: {}", e)))?;

        // Extraer ticket si SUNAT lo devolvió en la cabecera o cuerpo
        let ticket = respuesta
            .headers()
            .get("numTicket")
            .or_else(|| respuesta.headers().get("ticket"))
            .and_then(|h| h.to_str().ok().map(|s| s.to_string()));

        Ok((nuevo_offset, ticket))
    }

    /// Orquesta la subida completa del archivo en fragmentos, reanudando automáticamente
    /// si existe un desplazamiento previo y emitiendo el progreso opcionalmente.
    #[instrument(skip(self, token_bearer, bytes_archivo, metadatos, callback_progreso), level = "info")]
    pub async fn subir_archivo<F>(
        &self,
        endpoint_creacion: &str,
        token_bearer: &str,
        bytes_archivo: &[u8],
        metadatos: &SireMetadatosTus,
        mut callback_progreso: Option<F>,
    ) -> SireResultado<SireRespuestaTus>
    where
        F: FnMut(SireProgresoTus),
    {
        let longitud_total = bytes_archivo.len() as u64;

        // 1. Crear sesión de carga en SUNAT
        let url_upload = self
            .crear_sesion_upload(endpoint_creacion, token_bearer, longitud_total, metadatos)
            .await?;

        // 2. Consultar offset inicial
        let mut offset_actual = self.consultar_offset(&url_upload, token_bearer).await?;

        let tamano_chunk = self.configuracion.tamano_chunk_bytes;
        let total_fragmentos = ((longitud_total as usize).div_ceil(tamano_chunk)).max(1);
        let mut num_ticket_final = None;

        info!(
            total_bytes = %longitud_total,
            offset_inicial = %offset_actual,
            fragmentos = %total_fragmentos,
            "Iniciando transmisión de fragmentos TUS"
        );

        // 3. Transmisión iterativa de chunks
        while offset_actual < longitud_total {
            let inicio = offset_actual as usize;
            let fin = (inicio + tamano_chunk).min(bytes_archivo.len());
            let chunk = &bytes_archivo[inicio..fin];

            let mut intentos = 0;
            let mut enviado = false;

            while intentos <= self.configuracion.max_reintentos_chunk {
                match self.enviar_chunk(&url_upload, token_bearer, offset_actual, chunk).await {
                    Ok((nuevo_offset, ticket)) => {
                        if ticket.is_some() {
                            num_ticket_final = ticket;
                        }

                        if nuevo_offset <= offset_actual {
                            return Err(SireError::Tus(format!(
                                "Inconsistencia de offset TUS: nuevo offset ({}) no avanzó desde ({})",
                                nuevo_offset, offset_actual
                            )));
                        }

                        offset_actual = nuevo_offset;
                        enviado = true;

                        let fragmento_indice = (offset_actual as usize).div_ceil(tamano_chunk);
                        let progreso = SireProgresoTus::calcular(
                            offset_actual,
                            longitud_total,
                            fragmento_indice,
                            total_fragmentos,
                        );

                        if let Some(ref mut cb) = callback_progreso {
                            cb(progreso);
                        }

                        break;
                    }
                    Err(e) => {
                        intentos += 1;
                        if intentos > self.configuracion.max_reintentos_chunk {
                            error!(intento = %intentos, error = ?e, "Superado máximo de reintentos para chunk TUS");
                            return Err(e);
                        }

                        warn!(
                            intento = %intentos,
                            error = ?e,
                            "Reintentando fragmento TUS tras error transitorio"
                        );
                        let espera = self.configuracion.espera_reintento_base * intentos;
                        tokio::time::sleep(espera).await;

                        // Al reintentar, volver a consultar el offset exacto del servidor
                        if let Ok(offset_recuperado) = self.consultar_offset(&url_upload, token_bearer).await {
                            offset_actual = offset_recuperado;
                        }
                    }
                }
            }

            if !enviado {
                return Err(SireError::Tus(format!(
                    "No fue posible completar la transmisión del fragmento en offset {}",
                    offset_actual
                )));
            }
        }

        info!(
            url = %url_upload,
            total_bytes = %offset_actual,
            "Carga TUS completada exitosamente al 100%"
        );

        Ok(SireRespuestaTus {
            url_upload,
            bytes_subidos: offset_actual,
            num_ticket: num_ticket_final,
        })
    }
}
