// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Gestor Asíncrono de Tokens OAuth 2.0 en Memoria
//!
//! Administra la obtención, cacheo concurrente y auto-refresco transparente del token
//! de acceso de SUNAT mediante `tokio::sync::RwLock`, evitando bloqueos de hilos y
//! múltiples solicitudes simultáneas de renovación.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, instrument};

use crate::sire_autenticacion::sire_ambiente::SireAmbiente;
use crate::sire_autenticacion::sire_credenciales::SireCredenciales;
use crate::sire_autenticacion::sire_token::SireToken;
use crate::sire_errores::{SireError, SireResultado};

/// Administrador de tokens en memoria RAM con control de concurrencia para Tokio.
#[derive(Debug, Clone)]
pub struct SireGestorToken {
    credenciales: Arc<SireCredenciales>,
    ambiente: SireAmbiente,
    token_en_cache: Arc<RwLock<Option<SireToken>>>,
    margen_expiracion_segundos: i64,
}

impl SireGestorToken {
    /// Inicializa un nuevo gestor de tokens con las credenciales y ambiente dados.
    pub fn nuevo(credenciales: SireCredenciales, ambiente: SireAmbiente) -> Self {
        Self {
            credenciales: Arc::new(credenciales),
            ambiente,
            token_en_cache: Arc::new(RwLock::new(None)),
            margen_expiracion_segundos: 60, // Refrescar 60 segundos antes del vencimiento
        }
    }

    /// Retorna el token de acceso válido actual, solicitando uno nuevo al servidor de SUNAT
    /// únicamente si no existe o si está próximo a expirar.
    #[instrument(skip(self, cliente_http), level = "debug")]
    pub async fn obtener_token_valido(&self, cliente_http: &reqwest::Client) -> SireResultado<String> {
        // 1. Lectura rápida con lock compartido
        {
            let lock_lectura = self.token_en_cache.read().await;
            if let Some(token) = lock_lectura.as_ref() {
                if !token.esta_expirado_con_margen(self.margen_expiracion_segundos) {
                    debug!("Token en caché vigente encontrado");
                    return Ok(token.access_token.clone());
                }
            }
        }

        // 2. Si no existe o expiró, adquirir lock exclusivo para refrescar
        let mut lock_escritura = self.token_en_cache.write().await;

        // Doble comprobación: otra tarea pudo haber renovado el token mientras esperábamos el lock
        if let Some(token) = lock_escritura.as_ref() {
            if !token.esta_expirado_con_margen(self.margen_expiracion_segundos) {
                return Ok(token.access_token.clone());
            }
        }

        info!("Solicitando nuevo token de acceso a SUNAT OAuth 2.0");
        let nuevo_token = self.solicitar_nuevo_token(cliente_http).await?;
        let access_token = nuevo_token.access_token.clone();
        *lock_escritura = Some(nuevo_token);

        Ok(access_token)
    }

    /// Ejecuta la petición POST de autenticación contra el servidor OAuth 2.0 de SUNAT.
    async fn solicitar_nuevo_token(&self, cliente_http: &reqwest::Client) -> SireResultado<SireToken> {
        let url = self.ambiente.construir_url_token(&self.credenciales.client_id);

        let mut parametros = HashMap::new();
        parametros.insert("grant_type", "password");
        parametros.insert("scope", "https://api-sire.sunat.gob.pe");
        parametros.insert("client_id", self.credenciales.client_id.as_str());
        parametros.insert("client_secret", self.credenciales.client_secret.as_str());

        let username = self.credenciales.username_compuesto();
        parametros.insert("username", username.as_str());
        parametros.insert("password", self.credenciales.clave_sol.as_str());

        let respuesta = cliente_http
            .post(&url)
            .form(&parametros)
            .send()
            .await
            .map_err(SireError::Red)?;

        let estado = respuesta.status();
        if !estado.is_success() {
            let cuerpo = respuesta.text().await.unwrap_or_default();
            return Err(SireError::RespuestaSunat {
                codigo: estado.as_str().to_string(),
                mensaje: format!("Error en autenticación OAuth SUNAT: {}", cuerpo),
            });
        }

        let token_respuesta: SireToken = respuesta.json().await.map_err(SireError::Red)?;
        Ok(token_respuesta)
    }

    /// Invalida manualmente el token almacenado en caché, forzando una renovación en la siguiente llamada.
    pub async fn invalidar_cache(&self) {
        let mut lock_escritura = self.token_en_cache.write().await;
        *lock_escritura = None;
    }
}
