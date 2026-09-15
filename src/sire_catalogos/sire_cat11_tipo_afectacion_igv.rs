// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Catálogo 11 de SUNAT: Código de Tipo de Afectación al IGV
//!
//! Clasifica el tratamiento tributario frente al Impuesto General a las Ventas (IGV)
//! para cada línea o comprobante en las propuestas del RVIE y RCE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos oficiales del Catálogo 11 de SUNAT para Afectación al IGV.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireCatalogo11TipoAfectacionIgv {
    /// Gravado - Operación Onerosa.
    #[default]
    #[serde(rename = "10")]
    GravadoOperacionOnerosa,

    /// Gravado - Retiro por premio.
    #[serde(rename = "11")]
    GravadoRetiroPorPremio,

    /// Gravado - Retiro por donación.
    #[serde(rename = "12")]
    GravadoRetiroPorDonacion,

    /// Gravado - Retiro general.
    #[serde(rename = "13")]
    GravadoRetiro,

    /// Gravado - Retiro por publicidad.
    #[serde(rename = "14")]
    GravadoRetiroPorPublicidad,

    /// Gravado - Bonificaciones.
    #[serde(rename = "15")]
    GravadoBonificaciones,

    /// Gravado - Retiro por entrega a trabajadores.
    #[serde(rename = "16")]
    GravadoRetiroPorEntregaATrabajadores,

    /// Gravado - IVAP (Impuesto a la Venta de Arroz Pilado).
    #[serde(rename = "17")]
    GravadoIvap,

    /// Exonerado - Operación Onerosa.
    #[serde(rename = "20")]
    ExoneradoOperacionOnerosa,

    /// Exonerado - Transferencia Gratuita.
    #[serde(rename = "21")]
    ExoneradoTransferenciaGratuita,

    /// Inafecto - Operación Onerosa.
    #[serde(rename = "30")]
    InafectoOperacionOnerosa,

    /// Inafecto - Retiro por Bonificación.
    #[serde(rename = "31")]
    InafectoRetiroPorBonificacion,

    /// Inafecto - Retiro general.
    #[serde(rename = "32")]
    InafectoRetiro,

    /// Inafecto - Retiro por Muestras Médicas.
    #[serde(rename = "33")]
    InafectoRetiroPorMuestrasMedicas,

    /// Inafecto - Retiro por Convenio Colectivo.
    #[serde(rename = "34")]
    InafectoRetiroPorConvenioColectivo,

    /// Inafecto - Retiro por Premio.
    #[serde(rename = "35")]
    InafectoRetiroPorPremio,

    /// Inafecto - Retiro por Publicidad.
    #[serde(rename = "36")]
    InafectoRetiroPorPublicidad,

    /// Exportación de Bienes o Servicios.
    #[serde(rename = "40")]
    Exportacion,
}

impl SireCatalogo11TipoAfectacionIgv {
    /// Retorna el código numérico de 2 dígitos de SUNAT.
    pub const fn codigo(&self) -> &'static str {
        match self {
            Self::GravadoOperacionOnerosa => "10",
            Self::GravadoRetiroPorPremio => "11",
            Self::GravadoRetiroPorDonacion => "12",
            Self::GravadoRetiro => "13",
            Self::GravadoRetiroPorPublicidad => "14",
            Self::GravadoBonificaciones => "15",
            Self::GravadoRetiroPorEntregaATrabajadores => "16",
            Self::GravadoIvap => "17",
            Self::ExoneradoOperacionOnerosa => "20",
            Self::ExoneradoTransferenciaGratuita => "21",
            Self::InafectoOperacionOnerosa => "30",
            Self::InafectoRetiroPorBonificacion => "31",
            Self::InafectoRetiro => "32",
            Self::InafectoRetiroPorMuestrasMedicas => "33",
            Self::InafectoRetiroPorConvenioColectivo => "34",
            Self::InafectoRetiroPorPremio => "35",
            Self::InafectoRetiroPorPublicidad => "36",
            Self::Exportacion => "40",
        }
    }

    /// Retorna la descripción oficial en español del tipo de afectación.
    pub const fn descripcion(&self) -> &'static str {
        match self {
            Self::GravadoOperacionOnerosa => "Gravado - Operación Onerosa",
            Self::GravadoRetiroPorPremio => "Gravado - Retiro por premio",
            Self::GravadoRetiroPorDonacion => "Gravado - Retiro por donación",
            Self::GravadoRetiro => "Gravado - Retiro",
            Self::GravadoRetiroPorPublicidad => "Gravado - Retiro por publicidad",
            Self::GravadoBonificaciones => "Gravado - Bonificaciones",
            Self::GravadoRetiroPorEntregaATrabajadores => "Gravado - Retiro por entrega a trabajadores",
            Self::GravadoIvap => "Gravado - IVAP",
            Self::ExoneradoOperacionOnerosa => "Exonerado - Operación Onerosa",
            Self::ExoneradoTransferenciaGratuita => "Exonerado - Transferencia Gratuita",
            Self::InafectoOperacionOnerosa => "Inafecto - Operación Onerosa",
            Self::InafectoRetiroPorBonificacion => "Inafecto - Retiro por Bonificación",
            Self::InafectoRetiro => "Inafecto - Retiro",
            Self::InafectoRetiroPorMuestrasMedicas => "Inafecto - Retiro por Muestras Médicas",
            Self::InafectoRetiroPorConvenioColectivo => "Inafecto - Retiro por Convenio Colectivo",
            Self::InafectoRetiroPorPremio => "Inafecto - Retiro por Premio",
            Self::InafectoRetiroPorPublicidad => "Inafecto - Retiro por Publicidad",
            Self::Exportacion => "Exportación de Bienes o Servicios",
        }
    }

    /// Determina si la afectación corresponde al grupo de operaciones gravadas con IGV.
    pub const fn es_gravado(&self) -> bool {
        matches!(
            self,
            Self::GravadoOperacionOnerosa
                | Self::GravadoRetiroPorPremio
                | Self::GravadoRetiroPorDonacion
                | Self::GravadoRetiro
                | Self::GravadoRetiroPorPublicidad
                | Self::GravadoBonificaciones
                | Self::GravadoRetiroPorEntregaATrabajadores
                | Self::GravadoIvap
        )
    }

    /// Determina si la afectación es exonerada de IGV.
    pub const fn es_exonerado(&self) -> bool {
        matches!(self, Self::ExoneradoOperacionOnerosa | Self::ExoneradoTransferenciaGratuita)
    }

    /// Determina si la afectación es inafecta al IGV.
    pub const fn es_inafecto(&self) -> bool {
        matches!(
            self,
            Self::InafectoOperacionOnerosa
                | Self::InafectoRetiroPorBonificacion
                | Self::InafectoRetiro
                | Self::InafectoRetiroPorMuestrasMedicas
                | Self::InafectoRetiroPorConvenioColectivo
                | Self::InafectoRetiroPorPremio
                | Self::InafectoRetiroPorPublicidad
        )
    }

    /// Determina si la afectación corresponde a exportación.
    pub const fn es_exportacion(&self) -> bool {
        matches!(self, Self::Exportacion)
    }
}

impl fmt::Display for SireCatalogo11TipoAfectacionIgv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
