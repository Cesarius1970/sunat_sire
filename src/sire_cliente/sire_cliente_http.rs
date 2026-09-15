// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Cliente HTTP Asíncrono para SUNAT SIRE
//!
//! Gestiona las peticiones REST contra los servicios de SUNAT SIRE con inyección
//! automática del Bearer token, reintentos y trazabilidad con `tracing`.

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use tracing::{debug, error, instrument};

use crate::sire_autenticacion::{SireCredenciales, SireGestorToken};
use crate::sire_cliente::sire_configuracion::SireConfiguracion;
use crate::sire_errores::{SireError, SireResultado};

/// Cliente HTTP principal para la interacción con los servicios de SUNAT SIRE.
#[derive(Debug, Clone)]
pub struct SireCliente {
    cliente_http: reqwest::Client,
    configuracion: SireConfiguracion,
    gestor_token: SireGestorToken,
}

impl SireCliente {
    /// Inicializa un nuevo cliente SIRE con las credenciales y configuración proporcionadas.
    pub fn nuevo(credenciales: SireCredenciales, configuracion: SireConfiguracion) -> SireResultado<Self> {
        let mut headers_por_defecto = HeaderMap::new();
        headers_por_defecto.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        let cliente_http = reqwest::Client::builder()
            .timeout(configuracion.timeout_solicitud)
            .connect_timeout(configuracion.timeout_conexion)
            .default_headers(headers_por_defecto)
            .build()
            .map_err(SireError::Red)?;

        let gestor_token = SireGestorToken::nuevo(credenciales, configuracion.ambiente.clone());

        Ok(Self {
            cliente_http,
            configuracion,
            gestor_token,
        })
    }

    /// Retorna una referencia a la configuración actual del cliente.
    pub fn configuracion(&self) -> &SireConfiguracion {
        &self.configuracion
    }

    /// Fuerza la autenticación y retorna el token Bearer emitido por SUNAT.
    pub async fn autenticar(&self) -> SireResultado<String> {
        self.gestor_token.obtener_token_valido(&self.cliente_http).await
    }

    /// Retorna el token Bearer vigente (en caché o renovado).
    pub async fn obtener_token(&self) -> SireResultado<String> {
        self.gestor_token.obtener_token_valido(&self.cliente_http).await
    }

    /// Ejecuta una solicitud HTTP GET autenticada contra un servicio del SIRE.
    #[instrument(skip(self), level = "debug")]
    pub async fn ejecutar_get(
        &self,
        ruta_servicio: &str,
        parametros_query: Option<&[(&str, &str)]>,
    ) -> SireResultado<reqwest::Response> {
        let token = self.obtener_token().await?;
        let url = self.configuracion.ambiente.construir_url_api(ruta_servicio);

        let mut peticion = self
            .cliente_http
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token));

        if let Some(query) = parametros_query {
            peticion = peticion.query(query);
        }

        debug!(url = %url, "Ejecutando GET en API SIRE");
        let respuesta = peticion.send().await.map_err(SireError::Red)?;

        let estado = respuesta.status();
        if !estado.is_success() {
            let cuerpo = respuesta.text().await.unwrap_or_default();
            error!(estado = %estado, cuerpo = %cuerpo, "Respuesta no exitosa de API SIRE");
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en API SIRE GET {}: {}", ruta_servicio, cuerpo),
            });
        }

        Ok(respuesta)
    }

    /// Ejecuta una solicitud HTTP POST con cuerpo JSON autenticado.
    #[instrument(skip(self, cuerpo), level = "debug")]
    pub async fn ejecutar_post_json<T: Serialize>(
        &self,
        ruta_servicio: &str,
        cuerpo: &T,
    ) -> SireResultado<reqwest::Response> {
        let token = self.obtener_token().await?;
        let url = self.configuracion.ambiente.construir_url_api(ruta_servicio);

        debug!(url = %url, "Ejecutando POST JSON en API SIRE");
        let respuesta = self
            .cliente_http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(cuerpo)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        if !estado.is_success() {
            let cuerpo_texto = respuesta.text().await.unwrap_or_default();
            error!(estado = %estado, cuerpo = %cuerpo_texto, "Respuesta no exitosa de API SIRE");
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en API SIRE POST {}: {}", ruta_servicio, cuerpo_texto),
            });
        }

        Ok(respuesta)
    }

    /// Ejecuta una subida de archivo ZIP comprimido (multipart/form-data) con hash SHA-256 según especificación SUNAT.
    #[instrument(skip(self, bytes_zip), level = "info")]
    pub async fn ejecutar_post_archivo_zip(
        &self,
        ruta_servicio: &str,
        nombre_archivo: &str,
        bytes_zip: Vec<u8>,
        sha256_hex: &str,
    ) -> SireResultado<reqwest::Response> {
        let token = self.obtener_token().await?;
        let url = self.configuracion.ambiente.construir_url_api(ruta_servicio);

        let parte_archivo = Part::bytes(bytes_zip)
            .file_name(nombre_archivo.to_string())
            .mime_str("application/zip")
            .map_err(|e| SireError::Archivo(format!("Error en tipo MIME: {}", e)))?;

        let formulario = Form::new()
            .part("archivo", parte_archivo)
            .text("nomArchivoImportacion", nombre_archivo.to_string())
            .text("hash", sha256_hex.to_string());

        debug!(url = %url, archivo = %nombre_archivo, hash = %sha256_hex, "Subiendo archivo ZIP al SIRE");

        let respuesta = self
            .cliente_http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .multipart(formulario)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        if !estado.is_success() {
            let cuerpo_texto = respuesta.text().await.unwrap_or_default();
            error!(estado = %estado, cuerpo = %cuerpo_texto, "Fallo en subida de archivo ZIP al SIRE");
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en subida de archivo ZIP al SIRE: {}", cuerpo_texto),
            });
        }

        Ok(respuesta)
    }

    /// Ejecuta una subida de archivo masiva y resumible utilizando el protocolo TUS 1.0.0.
    #[instrument(skip(self, bytes_archivo, callback_progreso), level = "info")]
    pub async fn ejecutar_upload_tus<F>(
        &self,
        ruta_servicio: &str,
        bytes_archivo: &[u8],
        metadatos: &crate::sire_tus::SireMetadatosTus,
        configuracion_tus: Option<crate::sire_tus::SireConfiguracionTus>,
        callback_progreso: Option<F>,
    ) -> SireResultado<crate::sire_tus::SireRespuestaTus>
    where
        F: FnMut(crate::sire_tus::SireProgresoTus),
    {
        let token = self.obtener_token().await?;
        let url_endpoint = self.configuracion.ambiente.construir_url_api(ruta_servicio);
        let config = configuracion_tus.unwrap_or_default();
        let cliente_tus = crate::sire_tus::SireClienteTus::nuevo(self.cliente_http.clone(), config);

        cliente_tus
            .subir_archivo(
                &url_endpoint,
                &token,
                bytes_archivo,
                metadatos,
                callback_progreso,
            )
            .await
    }
}


