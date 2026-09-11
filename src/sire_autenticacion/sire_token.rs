// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Representación y Ciclo de Vida del Token Bearer de SUNAT
//!
//! Almacena el token JWT retornado por el servidor OAuth 2.0 y calcula su validez temporal.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Estructura de respuesta y almacenamiento del token de acceso de SUNAT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SireToken {
    /// Token de acceso emitido (Bearer JWT).
    #[serde(rename = "access_token")]
    pub access_token: String,

    /// Tipo de token (habitualmente "bearer").
    #[serde(rename = "token_type", default = "default_token_type")]
    pub token_type: String,

    /// Tiempo de vida en segundos otorgado por SUNAT (ej. 3600).
    #[serde(rename = "expires_in")]
    pub expires_in: i64,

    /// Momento exacto UTC en el que se adquirió el token.
    #[serde(default = "Utc::now")]
    pub adquirido_en: DateTime<Utc>,
}

fn default_token_type() -> String {
    "bearer".to_string()
}

impl SireToken {
    /// Crea una nueva instancia fijando la fecha y hora de adquisición en el momento actual.
    pub fn nuevo(access_token: impl Into<String>, expires_in: i64) -> Self {
        Self {
            access_token: access_token.into(),
            token_type: default_token_type(),
            expires_in,
            adquirido_en: Utc::now(),
        }
    }

    /// Determina si el token ha expirado o está a punto de expirar dentro del margen de seguridad dado en segundos.
    ///
    /// # Parámetros
    /// - `margen_segundos`: Ventana de tiempo previa a la caducidad (ej. 60 segundos) para refrescar con antelación.
    pub fn esta_expirado_con_margen(&self, margen_segundos: i64) -> bool {
        let tiempo_transcurrido = Utc::now().signed_duration_since(self.adquirido_en);
        let tiempo_validez = Duration::seconds(self.expires_in - margen_segundos);
        tiempo_transcurrido >= tiempo_validez
    }

    /// Retorna el encabezado HTTP de autorización listo para su uso.
    pub fn encabezado_autorizacion(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}
