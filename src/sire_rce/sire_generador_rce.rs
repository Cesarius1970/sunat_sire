// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Generador de Archivos Planos y Empaquetado ZIP para RCE
//!
//! Exportación offline independiente de archivos `.txt` delimitados por `|`,
//! compresión en memoria `.zip` y cálculo del hash SHA-256 para el Registro de Compras.

use sha2::{Digest, Sha256};
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::sire_errores::{SireError, SireResultado};
use crate::sire_rce::sire_modelo_rce::SireComprobanteCompra;

/// Genera el contenido de texto plano correspondiente al reemplazo o propuesta del RCE.
pub fn sire_generar_archivo_plano_rce(comprobantes: &[SireComprobanteCompra]) -> String {
    let mut contenido = String::new();
    for comprobante in comprobantes {
        contenido.push_str(&comprobante.formatear_linea_plano());
        contenido.push_str("\r\n");
    }
    contenido
}

/// Genera el archivo ZIP en memoria para RCE y calcula su hash SHA-256.
///
/// # Retorno
/// Retorna `(nombre_archivo_zip, bytes_del_zip, hash_sha256_hex)`.
pub fn sire_empaquetar_zip_rce(
    ruc: &str,
    periodo: &str,
    contenido_plano: &str,
) -> SireResultado<(String, Vec<u8>, String)> {
    // Nomenclatura oficial SUNAT para reemplazo de propuesta RCE:
    // LE + RUC + Periodo(YYYYMM) + 00 + 080400 (RCE) + 02 (Reemplazo) + 1 (Con info) + 1 (Soles) + 1 (SIRE)
    let nombre_base = format!("LE{}{}{}021112", ruc.trim(), periodo.trim(), "00080400");
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

    let mut hasher = Sha256::new();
    hasher.update(&buffer_zip);
    let hash_sha256_hex = format!("{:x}", hasher.finalize());

    Ok((nombre_zip, buffer_zip, hash_sha256_hex))
}
