// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Ambientes de Ejecución para SUNAT SIRE
//!
//! Define las URLs base y configuraciones de entorno para la comunicación
//! con los servidores de autenticación y servicios web API REST de SUNAT.

use serde::{Deserialize, Serialize};

/// Constante para URL oficial de autenticación de SUNAT en Producción.
pub const URL_SEGURIDAD_PRODUCCION: &str = "https://api-seguridad.sunat.gob.pe";

/// Constante para URL oficial de la API SIRE de SUNAT en Producción.
pub const URL_API_SIRE_PRODUCCION: &str = "https://api-sire.sunat.gob.pe";

/// Constante para URL oficial de la API SIRE en ambiente de Pruebas / Beta.
pub const URL_API_SIRE_BETA: &str = "https://api-sire-beta.sunat.gob.pe";

/// Ambientes de despliegue admitidos para la interacción con SIRE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SireAmbiente {
    /// Ambiente real de producción de SUNAT.
    #[default]
    Produccion,

    /// Ambiente de pruebas / homologación oficial (Beta) de SUNAT.
    PruebasBeta,

    /// Ambiente con endpoints personalizados (ej. mock servers locales o proxies corporativos).
    Personalizado {
        /// URL base del servidor OAuth de seguridad.
        url_seguridad: String,
        /// URL base del API de servicios del SIRE.
        url_api: String,
    },
}

impl SireAmbiente {
    /// Retorna la URL base del servicio de seguridad y token OAuth 2.0.
    pub fn url_seguridad(&self) -> &str {
        match self {
            Self::Produccion | Self::PruebasBeta => URL_SEGURIDAD_PRODUCCION,
            Self::Personalizado { url_seguridad, .. } => url_seguridad.as_str(),
        }
    }

    /// Retorna la URL base del API de servicios del SIRE (MIGE).
    pub fn url_api(&self) -> &str {
        match self {
            Self::Produccion => URL_API_SIRE_PRODUCCION,
            Self::PruebasBeta => URL_API_SIRE_BETA,
            Self::Personalizado { url_api, .. } => url_api.as_str(),
        }
    }

    /// Genera la URL completa para el endpoint de obtención de token OAuth 2.0.
    pub fn construir_url_token(&self, client_id: &str) -> String {
        format!(
            "{}/v1/clientessol/{}/oauth2/token/",
            self.url_seguridad().trim_end_matches('/'),
            client_id.trim()
        )
    }

    /// Construye una URL completa combinando la base del API con el path del servicio.
    pub fn construir_url_api(&self, ruta_servicio: &str) -> String {
        let base = self.url_api().trim_end_matches('/');
        let ruta = ruta_servicio.trim_start_matches('/');
        format!("{}/{}", base, ruta)
    }
}
