// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Configuración del Cliente TUS (`sire_configuracion_tus`)
//!
//! Parámetros de configuración para la transmisión resumible de fragmentos
//! (tamaño de chunk, reintentos y timeouts).

use std::time::Duration;

/// Parámetros de configuración para el cliente de carga TUS.
#[derive(Debug, Clone)]
pub struct SireConfiguracionTus {
    /// Tamaño de cada fragmento (chunk) en bytes para las peticiones PATCH.
    /// Por defecto: 5 MB (5,242,880 bytes).
    pub tamano_chunk_bytes: usize,
    /// Número máximo de reintentos por fragmento ante fallos transitorios de red.
    pub max_reintentos_chunk: u32,
    /// Tiempo límite por solicitud de fragmento.
    pub timeout_chunk: Duration,
    /// Tiempo de espera base para el retroceso exponencial (backoff) entre reintentos.
    pub espera_reintento_base: Duration,
}

impl Default for SireConfiguracionTus {
    fn default() -> Self {
        Self {
            tamano_chunk_bytes: 5 * 1024 * 1024, // 5 MB
            max_reintentos_chunk: 3,
            timeout_chunk: Duration::from_secs(60),
            espera_reintento_base: Duration::from_millis(500),
        }
    }
}

impl SireConfiguracionTus {
    /// Crea una nueva configuración con valores predeterminados.
    pub fn nuevo() -> Self {
        Self::default()
    }

    /// Permite personalizar el tamaño de fragmento en megabytes.
    pub fn con_tamano_chunk_mb(mut self, mb: usize) -> Self {
        self.tamano_chunk_bytes = mb * 1024 * 1024;
        self
    }

    /// Permite personalizar el número máximo de reintentos por chunk.
    pub fn con_max_reintentos(mut self, reintentos: u32) -> Self {
        self.max_reintentos_chunk = reintentos;
        self
    }

    /// Permite personalizar el tiempo de espera por chunk.
    pub fn con_timeout_chunk(mut self, timeout: Duration) -> Self {
        self.timeout_chunk = timeout;
        self
    }
}
