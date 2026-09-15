// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Configuración del Cliente HTTP SIRE
//!
//! Ajustes de red, reintentos, tiempos de espera (*timeouts*) y ambiente de destino.

use crate::sire_autenticacion::SireAmbiente;
use std::time::Duration;

/// Opciones de configuración para el cliente HTTP asíncrono de SUNAT SIRE.
#[derive(Debug, Clone)]
pub struct SireConfiguracion {
    /// Ambiente de ejecución (Producción, Pruebas Beta o Personalizado).
    pub ambiente: SireAmbiente,

    /// Tiempo de espera total por solicitud HTTP.
    pub timeout_solicitud: Duration,

    /// Tiempo de espera para el establecimiento de la conexión TCP/TLS.
    pub timeout_conexion: Duration,

    /// Número máximo de reintentos ante errores transitorios de red.
    pub reintentos_maximos: u32,
}

impl Default for SireConfiguracion {
    fn default() -> Self {
        Self {
            ambiente: SireAmbiente::Produccion,
            timeout_solicitud: Duration::from_secs(60),
            timeout_conexion: Duration::from_secs(15),
            reintentos_maximos: 3,
        }
    }
}

impl SireConfiguracion {
    /// Crea una nueva configuración para el ambiente de Producción.
    pub fn produccion() -> Self {
        Self {
            ambiente: SireAmbiente::Produccion,
            ..Default::default()
        }
    }

    /// Crea una nueva configuración para el ambiente de Pruebas / Beta.
    pub fn pruebas_beta() -> Self {
        Self {
            ambiente: SireAmbiente::PruebasBeta,
            ..Default::default()
        }
    }

    /// Personaliza el tiempo de espera por solicitud.
    pub fn con_timeout_solicitud(mut self, timeout: Duration) -> Self {
        self.timeout_solicitud = timeout;
        self
    }
}
