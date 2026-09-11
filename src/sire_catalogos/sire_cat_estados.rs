// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Estados de Comprobantes, Propuestas y Tickets del SIRE
//!
//! Enumeraciones tipadas para el ciclo de vida de los comprobantes dentro de la propuesta,
//! los estados de procesamiento asíncrono de tickets de SUNAT y tipos de operaciones SIRE.

use serde::{Deserialize, Serialize};

/// Estado de un comprobante dentro de la propuesta de SUNAT (RVIE o RCE).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireEstadoComprobantePropuesta {
    /// Comprobante propuesto por SUNAT y aceptado sin modificaciones.
    #[default]
    #[serde(rename = "0")]
    Aceptado,

    /// Comprobante incorporado o complementado por el contribuyente.
    #[serde(rename = "1")]
    Incorporado,

    /// Comprobante modificado por el contribuyente.
    #[serde(rename = "2")]
    Modificado,

    /// Comprobante excluido de la propuesta del periodo (pospuesto o rechazado).
    #[serde(rename = "3")]
    Excluido,
}

/// Estado de procesamiento de un ticket asíncrono en SUNAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireEstadoTicket {
    /// Ticket registrado en la cola de procesamiento de SUNAT.
    #[default]
    #[serde(rename = "01")]
    Registrado,

    /// Ticket en proceso de validación / generación en SUNAT.
    #[serde(rename = "02")]
    EnProceso,

    /// Proceso finalizado exitosamente. Archivos listos para descarga.
    #[serde(rename = "03")]
    Terminado,

    /// Proceso finalizado con advertencias o errores en parte de los registros.
    #[serde(rename = "04")]
    TerminadoConErrores,

    /// Proceso rechazado totalmente por SUNAT (formato inválido, inconsistencia grave).
    #[serde(rename = "05")]
    Rechazado,
}

impl SireEstadoTicket {
    /// Indica si el ticket ya completó su ciclo de vida en SUNAT (éxito o fallo).
    pub const fn esta_completado(&self) -> bool {
        matches!(
            self,
            Self::Terminado | Self::TerminadoConErrores | Self::Rechazado
        )
    }

    /// Indica si el ticket concluyó de manera totalmente exitosa.
    pub const fn es_exitoso(&self) -> bool {
        matches!(self, Self::Terminado)
    }
}

/// Tipo de proceso o solicitud asíncrona enviada a SUNAT mediante Ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SireTipoProceso {
    /// Aceptación formal de la propuesta de RVIE.
    AceptacionPropuestaRvie,
    /// Reemplazo completo de la propuesta de RVIE con archivo plano/ZIP.
    ReemplazoPropuestaRvie,
    /// Aceptación formal de la propuesta de RCE.
    AceptacionPropuestaRce,
    /// Complementación de la propuesta de RCE con documentos adicionales.
    ComplementacionPropuestaRce,
    /// Reemplazo completo de la propuesta de RCE con archivo plano/ZIP.
    ReemplazoPropuestaRce,
    /// Descarga masiva de la propuesta preliminar.
    DescargaPropuesta,
    /// Ajustes posteriores a periodos ya cerrados en el SIRE.
    AjustesPosteriores,
}
