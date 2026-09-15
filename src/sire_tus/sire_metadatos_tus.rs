// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Metadatos para Cargas Resumibles TUS (`sire_metadatos_tus`)
//!
//! Modela y formatea los metadatos requeridos por la cabecera `Upload-Metadata`
//! del protocolo TUS 1.0.0 según las directrices técnicas de SUNAT SIRE.

use base64::prelude::*;

/// Metadatos asociados a un archivo en proceso de carga mediante TUS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SireMetadatosTus {
    /// Nombre oficial del archivo (ej. `LE2060000000120260900140400021112.zip`).
    pub nombre_archivo: String,
    /// Tipo MIME del archivo (ej. `application/zip`).
    pub tipo_archivo: String,
    /// Hash criptográfico SHA-256 en hexadecimal (64 caracteres).
    pub hash_sha256: String,
    /// Número de RUC del contribuyente.
    pub ruc: String,
    /// Periodo tributario en formato AAAAMM (ej. `202609`).
    pub periodo: String,
}

impl SireMetadatosTus {
    /// Crea una nueva instancia de metadatos TUS para SUNAT SIRE.
    pub fn nuevo(
        nombre_archivo: impl Into<String>,
        hash_sha256: impl Into<String>,
        ruc: impl Into<String>,
        periodo: impl Into<String>,
    ) -> Self {
        Self {
            nombre_archivo: nombre_archivo.into(),
            tipo_archivo: "application/zip".to_string(),
            hash_sha256: hash_sha256.into(),
            ruc: ruc.into(),
            periodo: periodo.into(),
        }
    }

    /// Serializa los metadatos al formato estándar del encabezado `Upload-Metadata` de TUS 1.0.0.
    ///
    /// La especificación exige pares `clave base64(valor)` separados por comas:
    /// `filename <base64>,filetype <base64>,hash <base64>,numRuc <base64>,perTributario <base64>`
    pub fn a_cabecera_upload_metadata(&self) -> String {
        let pares = [
            ("filename", self.nombre_archivo.as_bytes()),
            ("filetype", self.tipo_archivo.as_bytes()),
            ("hash", self.hash_sha256.as_bytes()),
            ("numRuc", self.ruc.as_bytes()),
            ("perTributario", self.periodo.as_bytes()),
        ];

        pares
            .iter()
            .map(|(clave, valor)| format!("{} {}", clave, BASE64_STANDARD.encode(valor)))
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debe_formatear_cabecera_upload_metadata_correctamente() {
        let metadatos = SireMetadatosTus::nuevo(
            "archivo_prueba.zip",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "20600000001",
            "202609",
        );

        let cabecera = metadatos.a_cabecera_upload_metadata();
        assert!(cabecera.contains("filename "));
        assert!(cabecera.contains("filetype "));
        assert!(cabecera.contains("hash "));
        assert!(cabecera.contains("numRuc "));
        assert!(cabecera.contains("perTributario "));

        // Verificar que el filename esté debidamente codificado en Base64
        let esperado_filename = BASE64_STANDARD.encode(b"archivo_prueba.zip");
        assert!(cabecera.contains(&format!("filename {}", esperado_filename)));
    }
}
