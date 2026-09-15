// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Catálogo 01 de SUNAT: Código de Tipo de Documento de Identidad
//!
//! Especifica los tipos de documentos de identidad reconocidos por SUNAT para
//! clientes, proveedores y contribuyentes en el SIRE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos oficiales del Catálogo 01 de SUNAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireCatalogo01TipoDocumentoIdentidad {
    /// Sin documento (operaciones no identificadas).
    #[serde(rename = "0")]
    SinDocumento,

    /// Documento Nacional de Identidad (DNI).
    #[default]
    #[serde(rename = "1")]
    Dni,

    /// Carnet de Extranjería.
    #[serde(rename = "4")]
    CarnetExtranjeria,

    /// Registro Único de Contribuyentes (RUC).
    #[serde(rename = "6")]
    Ruc,

    /// Pasaporte.
    #[serde(rename = "7")]
    Pasaporte,

    /// Cédula Diplomática de Identidad.
    #[serde(rename = "A")]
    CedulaDiplomatica,

    /// Documento de Identidad del País de Residencia (No Domiciliados).
    #[serde(rename = "B")]
    DocIdentidadPaisResidencia,

    /// Tax Identification Number (TIN) o equivalente internacional.
    #[serde(rename = "C")]
    TaxIdentificationNumber,

    /// Identificación tributaria expedida por el país de origen.
    #[serde(rename = "D")]
    IdentificacionTributariaPais,

    /// Otros tipos de documentos de identidad.
    #[serde(rename = "E")]
    Otros,
}

impl SireCatalogo01TipoDocumentoIdentidad {
    /// Retorna el código alfanumérico oficial exigido por SUNAT.
    pub const fn codigo(&self) -> &'static str {
        match self {
            Self::SinDocumento => "0",
            Self::Dni => "1",
            Self::CarnetExtranjeria => "4",
            Self::Ruc => "6",
            Self::Pasaporte => "7",
            Self::CedulaDiplomatica => "A",
            Self::DocIdentidadPaisResidencia => "B",
            Self::TaxIdentificationNumber => "C",
            Self::IdentificacionTributariaPais => "D",
            Self::Otros => "E",
        }
    }

    /// Retorna la descripción oficial en español del tipo de documento.
    pub const fn descripcion(&self) -> &'static str {
        match self {
            Self::SinDocumento => "Doc. Trib. No Domiciliado Sin RUC / Sin Documento",
            Self::Dni => "Documento Nacional de Identidad (DNI)",
            Self::CarnetExtranjeria => "Carnet de Extranjería",
            Self::Ruc => "Registro Único de Contribuyentes (RUC)",
            Self::Pasaporte => "Pasaporte",
            Self::CedulaDiplomatica => "Cédula Diplomática de Identidad",
            Self::DocIdentidadPaisResidencia => "Documento de Identidad del País de Residencia",
            Self::TaxIdentificationNumber => "Tax Identification Number (TIN)",
            Self::IdentificacionTributariaPais => "Número de Identificación Tributaria del Exterior",
            Self::Otros => "Otros Documentos",
        }
    }

    /// Parsea un código de texto de SUNAT al tipo enumerado.
    pub fn desde_codigo(codigo: &str) -> Option<Self> {
        match codigo.trim() {
            "0" => Some(Self::SinDocumento),
            "1" => Some(Self::Dni),
            "4" => Some(Self::CarnetExtranjeria),
            "6" => Some(Self::Ruc),
            "7" => Some(Self::Pasaporte),
            "A" | "a" => Some(Self::CedulaDiplomatica),
            "B" | "b" => Some(Self::DocIdentidadPaisResidencia),
            "C" | "c" => Some(Self::TaxIdentificationNumber),
            "D" | "d" => Some(Self::IdentificacionTributariaPais),
            "E" | "e" => Some(Self::Otros),
            _ => None,
        }
    }
}

impl fmt::Display for SireCatalogo01TipoDocumentoIdentidad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
