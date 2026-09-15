// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo de Errores Tipados de SIRE (`sire_errores`)
//!
//! Proporciona la jerarquía de errores para todas las operaciones con el Sistema
//! Integrado de Registros Electrónicos (SIRE) de SUNAT, construida sobre `thiserror`.

use thiserror::Error;

/// Tipo de resultado estándar para operaciones del cliente SIRE.
pub type SireResultado<T> = Result<T, SireError>;

/// Jerarquía unificada de errores del sistema SIRE.
#[derive(Debug, Error)]
pub enum SireError {
    /// Error en el proceso de autenticación o renovación de token con SUNAT.
    #[error("Error de autenticación con SUNAT: {0}")]
    Autenticacion(String),

    /// Error de comunicación de red o cliente HTTP.
    #[error("Error HTTP/Red: {0}")]
    Red(#[from] reqwest::Error),

    /// Respuesta de error devuelta por los servicios web de SUNAT.
    #[error("Error devuelto por SUNAT (Código {codigo}): {mensaje}")]
    RespuestaSunat {
        /// Código de error emitido por SUNAT (ej. "401", "0001", "WRN-1002").
        codigo: String,
        /// Mensaje descriptivo retornado por SUNAT.
        mensaje: String,
    },

    /// Error de validación local o inconsistencia tributaria previa al envío.
    #[error("Error de validación local: {0}")]
    Validacion(String),

    /// Error en la lectura, escritura, compresión o descompresión de archivos planos / ZIP.
    #[error("Error de archivo o formato: {0}")]
    Archivo(String),

    /// Error de entrada/salida estándar.
    #[error("Error de E/S (I/O): {0}")]
    Io(#[from] std::io::Error),

    /// Error de serialización o deserialización de datos JSON.
    #[error("Error de serialización JSON: {0}")]
    Json(#[from] serde_json::Error),

    /// Error en operaciones con archivos ZIP.
    #[error("Error en archivo comprimido ZIP: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Error en conversión o cálculo decimal monetario.
    #[error("Error de precisión decimal: {0}")]
    Decimal(#[from] rust_decimal::Error),

    /// Error en operaciones del protocolo TUS (cargas masivas resumibles).
    #[error("Error en protocolo TUS: {0}")]
    Tus(String),
}

