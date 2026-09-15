// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Servicios e Integración con Propuestas de RCE
//!
//! Operaciones de consulta, aceptación, complementación y reemplazo de propuesta
//! de compras contra los servicios web API REST de SUNAT SIRE.

use serde::{Deserialize, Serialize};

use crate::sire_cliente::SireCliente;
use crate::sire_errores::{SireError, SireResultado};
use crate::sire_rce::sire_generador_rce::{sire_empaquetar_zip_rce, sire_generar_archivo_plano_rce};
use crate::sire_rce::sire_modelo_rce::SireComprobanteCompra;
use crate::sire_tickets::SireTicket;

/// Respuesta paginada de consulta de propuesta de compras (RCE).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SireRespuestaPropuestaRce {
    /// Total de registros disponibles en la propuesta.
    #[serde(rename = "totalRegistros", default)]
    pub total_registros: u64,

    /// Número de página consultada.
    #[serde(rename = "numPagina", default)]
    pub num_pagina: u32,

    /// Límite de registros por página.
    #[serde(rename = "numRegistrosPorPagina", default)]
    pub registros_por_pagina: u32,

    /// Comprobantes de compra en formato JSON devueltos por SUNAT.
    #[serde(rename = "registros", default)]
    pub registros: Vec<serde_json::Value>,
}

/// Consulta los comprobantes contenidos en la propuesta de RCE para un periodo tributario.
pub async fn sire_consultar_propuesta_rce(
    cliente: &SireCliente,
    periodo: &str,
    pagina: u32,
    limite: u32,
) -> SireResultado<SireRespuestaPropuestaRce> {
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/comprobantescompras?numPagina={}&numRegistrosPorPagina={}",
        periodo.trim(),
        pagina,
        limite
    );

    let respuesta = cliente.ejecutar_get(&ruta, None).await?;
    let resultado: SireRespuestaPropuestaRce = respuesta.json().await.map_err(SireError::Red)?;
    Ok(resultado)
}

/// Acepta formalmente la propuesta de compras (RCE) de SUNAT sin cambios.
pub async fn sire_aceptar_propuesta_rce(
    cliente: &SireCliente,
    periodo: &str,
) -> SireResultado<SireTicket> {
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/aceptarcompras",
        periodo.trim()
    );

    let respuesta = cliente.ejecutar_post_json(&ruta, &serde_json::json!({})).await?;
    let ticket: SireTicket = respuesta.json().await.map_err(SireError::Red)?;
    Ok(ticket)
}

/// Reemplaza la propuesta de compras empaquetando los comprobantes en un archivo ZIP con hash SHA-256.
pub async fn sire_reemplazar_propuesta_rce(
    cliente: &SireCliente,
    ruc: &str,
    periodo: &str,
    comprobantes: &[SireComprobanteCompra],
) -> SireResultado<SireTicket> {
    let plano = sire_generar_archivo_plano_rce(comprobantes);
    let (nombre_zip, bytes_zip, sha256_hex) = sire_empaquetar_zip_rce(ruc, periodo, &plano)?;

    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/reemplazarcompras",
        periodo.trim()
    );

    let respuesta = cliente
        .ejecutar_post_archivo_zip(&ruta, &nombre_zip, bytes_zip, &sha256_hex)
        .await?;

    let ticket: SireTicket = respuesta.json().await.map_err(SireError::Red)?;
    Ok(ticket)
}

/// Reemplaza la propuesta de compras (RCE) utilizando el protocolo TUS 1.0.0 para cargas resumibles masivas.
pub async fn sire_reemplazar_propuesta_rce_tus<F>(
    cliente: &SireCliente,
    ruc: &str,
    periodo: &str,
    comprobantes: &[SireComprobanteCompra],
    configuracion_tus: Option<crate::sire_tus::SireConfiguracionTus>,
    callback_progreso: Option<F>,
) -> SireResultado<crate::sire_tus::SireRespuestaTus>
where
    F: FnMut(crate::sire_tus::SireProgresoTus),
{
    let plano = sire_generar_archivo_plano_rce(comprobantes);
    let (nombre_zip, bytes_zip, sha256_hex) = sire_empaquetar_zip_rce(ruc, periodo, &plano)?;

    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/propuesta/web/propuesta/{}/reemplazarcompras/upload",
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


