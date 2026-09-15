// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Generador de Archivos Planos y Empaquetado ZIP para RVIE
//!
//! Permite la exportación offline e independiente de archivos `.txt` delimitados por `|`,
//! su compresión en memoria en formato `.zip` y el cálculo del hash criptográfico SHA-256.

use sha2::{Digest, Sha256};
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::sire_errores::{SireError, SireResultado};
use crate::sire_rvie::sire_modelo_rvie::SireComprobanteVenta;

/// Genera el contenido de texto plano correspondiente al reemplazo de propuesta RVIE.
pub fn sire_generar_archivo_plano_rvie(comprobantes: &[SireComprobanteVenta]) -> String {
    let mut contenido = String::new();
    for comprobante in comprobantes {
        contenido.push_str(&comprobante.formatear_linea_plano());
        contenido.push_str("\r\n");
    }
    contenido
}

/// Genera el archivo ZIP en memoria y calcula su hash SHA-256 oficial para SUNAT.
///
/// # Retorno
/// Retorna una tupla con `(nombre_archivo_zip, bytes_del_zip, hash_sha256_hex)`.
pub fn sire_empaquetar_zip_rvie(
    ruc: &str,
    periodo: &str,
    contenido_plano: &str,
) -> SireResultado<(String, Vec<u8>, String)> {
    // Nomenclatura oficial SUNAT para reemplazo de propuesta RVIE:
    // LE + RUC + Periodo(YYYYMM) + 00 + 140400 (RVIE) + 02 (Reemplazo) + 1 (Con info) + 1 (Soles) + 1 (SIRE)
    let nombre_base = format!("LE{}{}{}021112", ruc.trim(), periodo.trim(), "00140400");
    let nombre_txt = format!("{}.txt", nombre_base);
    let nombre_zip = format!("{}.zip", nombre_base);

    let mut buffer_zip = Vec::new();
    {
        let mut zip_writer = ZipWriter::new(std::io::Cursor::new(&mut buffer_zip));
        let opciones = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip_writer
            .start_file(nombre_txt, opciones)
            .map_err(SireError::Zip)?;

        zip_writer
            .write_all(contenido_plano.as_bytes())
            .map_err(SireError::Io)?;

        zip_writer.finish().map_err(SireError::Zip)?;
    }

    // Cálculo del hash SHA-256 sobre los bytes del archivo ZIP
    let mut hasher = Sha256::new();
    hasher.update(&buffer_zip);
    let hash_sha256_hex = format!("{:x}", hasher.finalize());

    Ok((nombre_zip, buffer_zip, hash_sha256_hex))
}
