// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Servicios e Integración con Propuestas de RVIE
//!
//! Implementa las operaciones de consulta, aceptación y reemplazo de propuesta de ventas
//! contra los servicios web API REST oficiales de SUNAT SIRE.

use serde::{Deserialize, Serialize};

use crate::sire_cliente::SireCliente;
use crate::sire_errores::{SireError, SireResultado};
use crate::sire_rvie::sire_generador_rvie::{sire_empaquetar_zip_rvie, sire_generar_archivo_plano_rvie};
use crate::sire_rvie::sire_modelo_rvie::SireComprobanteVenta;
use crate::sire_tickets::SireTicket;

/// Respuesta paginada de consulta de propuesta de comprobantes de ventas.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SireRespuestaPropuestaRvie {
    /// Número total de comprobantes en la propuesta.
    #[serde(rename = "totalRegistros", default)]
    pub total_registros: u64,

    /// Número de página consultada.
    #[serde(rename = "numPagina", default)]
    pub num_pagina: u32,

    /// Cantidad de registros devueltos por página.
    #[serde(rename = "numRegistrosPorPagina", default)]
    pub registros_por_pagina: u32,

    /// Lista de comprobantes en formato JSON devueltos por SUNAT.
    #[serde(rename = "registros", default)]
    pub registros: Vec<serde_json::Value>,
}

/// Consulta los comprobantes contenidos en la propuesta de RVIE para un periodo tributario.
pub async fn sire_consultar_propuesta_rvie(
    cliente: &SireCliente,
    periodo: &str,
    pagina: u32,
    limite: u32,
) -> SireResultado<SireRespuestaPropuestaRvie> {
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/comprobantes?numPagina={}&numRegistrosPorPagina={}",
        periodo.trim(),
        pagina,
        limite
    );

    let respuesta = cliente.ejecutar_get(&ruta, None).await?;
    let resultado: SireRespuestaPropuestaRvie = respuesta.json().await.map_err(SireError::Red)?;
    Ok(resultado)
}

/// Acepta formalmente la propuesta de RVIE de SUNAT sin cambios, generando un ticket de cierre.
pub async fn sire_aceptar_propuesta_rvie(
    cliente: &SireCliente,
    periodo: &str,
) -> SireResultado<SireTicket> {
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/aceptar",
        periodo.trim()
    );

    let respuesta = cliente.ejecutar_post_json(&ruta, &serde_json::json!({})).await?;
    let ticket: SireTicket = respuesta.json().await.map_err(SireError::Red)?;
    Ok(ticket)
}

/// Reemplaza la propuesta de ventas de SUNAT empaquetando los comprobantes en un archivo ZIP con hash SHA-256.
pub async fn sire_reemplazar_propuesta_rvie(
    cliente: &SireCliente,
    ruc: &str,
    periodo: &str,
    comprobantes: &[SireComprobanteVenta],
) -> SireResultado<SireTicket> {
    // 1. Generación del archivo plano en formato estándar
    let plano = sire_generar_archivo_plano_rvie(comprobantes);

    // 2. Compresión en memoria ZIP y cálculo SHA-256
    let (nombre_zip, bytes_zip, sha256_hex) = sire_empaquetar_zip_rvie(ruc, periodo, &plano)?;

    // 3. Subida del archivo a SUNAT SIRE
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/reemplazar",
        periodo.trim()
    );

    let respuesta = cliente
        .ejecutar_post_archivo_zip(&ruta, &nombre_zip, bytes_zip, &sha256_hex)
        .await?;

    let ticket: SireTicket = respuesta.json().await.map_err(SireError::Red)?;
    Ok(ticket)
}

/// Reemplaza la propuesta de ventas de SUNAT utilizando el protocolo TUS 1.0.0 para cargas resumibles masivas.
pub async fn sire_reemplazar_propuesta_rvie_tus<F>(
    cliente: &SireCliente,
    ruc: &str,
    periodo: &str,
    comprobantes: &[SireComprobanteVenta],
    configuracion_tus: Option<crate::sire_tus::SireConfiguracionTus>,
    callback_progreso: Option<F>,
) -> SireResultado<crate::sire_tus::SireRespuestaTus>
where
    F: FnMut(crate::sire_tus::SireProgresoTus),
{
    let plano = sire_generar_archivo_plano_rvie(comprobantes);
    let (nombre_zip, bytes_zip, sha256_hex) = sire_empaquetar_zip_rvie(ruc, periodo, &plano)?;

    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/reemplazar/upload",
        periodo.trim()
    );

    let metadatos = crate::sire_tus::SireMetadatosTus::nuevo(&nombre_zip, &sha256_hex, ruc, periodo);

    cliente
        .ejecutar_upload_tus(
            &ruta,
            &bytes_zip,
            &metadatos,
            configuracion_tus,
            callback_progreso,
        )
        .await
}


