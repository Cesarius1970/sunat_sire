// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Modelo de Progreso para Cargas TUS (`sire_progreso_tus`)
//!
//! Representa el estado de avance de la transferencia de un archivo masivo.

/// Información de progreso emitida durante la subida de fragmentos TUS.
#[derive(Debug, Clone, PartialEq)]
pub struct SireProgresoTus {
    /// Cantidad de bytes transferidos y confirmados por el servidor hasta el momento.
    pub bytes_transferidos: u64,
    /// Tamaño total del archivo en bytes.
    pub bytes_totales: u64,
    /// Porcentaje completado (0.0 a 100.0).
    pub porcentaje: f64,
    /// Índice del fragmento actual (1-indexed).
    pub fragmento_actual: usize,
    /// Total estimado de fragmentos a enviar.
    pub total_fragmentos: usize,
}

impl SireProgresoTus {
    /// Calcula un nuevo estado de progreso a partir de los bytes y fragmentos.
    pub fn calcular(
        bytes_transferidos: u64,
        bytes_totales: u64,
        fragmento_actual: usize,
        total_fragmentos: usize,
    ) -> Self {
        let porcentaje = if bytes_totales == 0 {
            100.0
        } else {
            (bytes_transferidos as f64 / bytes_totales as f64) * 100.0
        };

        Self {
            bytes_transferidos,
            bytes_totales,
            porcentaje: porcentaje.min(100.0),
            fragmento_actual,
            total_fragmentos,
        }
    }
}
