// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Modelos de Consulta y Estado de Tickets de SUNAT SIRE
//!
//! Estructuras tipadas para la gestión de solicitudes asíncronas, estado de
//! procesamiento y archivos de respuesta generados por SUNAT.

use crate::sire_catalogos::SireEstadoTicket;
use serde::{Deserialize, Serialize};

/// Información detallada de un ticket devuelta por SUNAT SIRE.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SireTicket {
    /// Número de ticket único asignado por SUNAT.
    #[serde(rename = "numTicket", alias = "num_ticket", default)]
    pub num_ticket: String,

    /// Fecha y hora de generación del ticket.
    #[serde(rename = "fecCargaImportacion", alias = "fecTicket", default)]
    pub fecha_registro: Option<String>,

    /// Código de proceso (ej. "01" RVIE propuesta, "02" RCE reemplazo).
    #[serde(rename = "codProceso", alias = "cod_proceso", default)]
    pub codigo_proceso: Option<String>,

    /// Código de estado devuelto por el API (ej. "01", "02", "03", "04", "05").
    #[serde(rename = "codEstadoProceso", alias = "cod_estado", default)]
    pub codigo_estado: Option<String>,

    /// Descripción textual del estado del proceso.
    #[serde(rename = "desEstadoProceso", alias = "des_estado", default)]
    pub descripcion_estado: Option<String>,

    /// Lista de archivos resultantes disponibles para descarga.
    #[serde(rename = "archivos", default)]
    pub archivos: Vec<SireArchivoRespuesta>,
}

impl SireTicket {
    /// Determina el estado tipado del ticket.
    pub fn estado_tipado(&self) -> SireEstadoTicket {
        match self.codigo_estado.as_deref().unwrap_or("").trim() {
            "01" => SireEstadoTicket::Registrado,
            "02" => SireEstadoTicket::EnProceso,
            "03" => SireEstadoTicket::Terminado,
            "04" => SireEstadoTicket::TerminadoConErrores,
            "05" => SireEstadoTicket::Rechazado,
            _ => {
                // Si la descripción contiene palabras clave
                if let Some(des) = &self.descripcion_estado {
                    let min = des.to_lowercase();
                    if min.contains("terminado") || min.contains("concluido") {
                        return SireEstadoTicket::Terminado;
                    }
                    if min.contains("error") || min.contains("rechaz") {
                        return SireEstadoTicket::Rechazado;
                    }
                    if min.contains("proceso") {
                        return SireEstadoTicket::EnProceso;
                    }
                }
                SireEstadoTicket::Registrado
            }
        }
    }

    /// Indica si el proceso del ticket ha concluido en SUNAT.
    pub fn esta_completado(&self) -> bool {
        self.estado_tipado().esta_completado()
    }
}

/// Detalle de archivo de salida generado tras el procesamiento de un ticket.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SireArchivoRespuesta {
    /// Nombre oficial del archivo generado por SUNAT.
    #[serde(rename = "nomArchivo", default)]
    pub nombre_archivo: String,

    /// URL o endpoint relativo para la descarga del archivo ZIP / reporte.
    #[serde(rename = "urlDescarga", alias = "nomRutaDescarga", default)]
    pub ruta_descarga: String,

    /// Número de registros procesados con éxito.
    #[serde(rename = "numRegistrosAceptados", default)]
    pub registros_aceptados: Option<u64>,

    /// Número de registros rechazados o con inconsistencias.
    #[serde(rename = "numRegistrosRechazados", default)]
    pub registros_rechazados: Option<u64>,
}
