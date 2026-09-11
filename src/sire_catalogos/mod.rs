// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo de Catálogos Oficiales de SUNAT para SIRE (`sire_catalogos`)
//!
//! Contiene las definiciones tipadas y métodos auxiliares para todas las tablas maestras
//! y catálogos normalizados según las especificaciones técnicas del SIRE (SUNAT).

pub mod sire_cat01_tipo_doc_identidad;
pub mod sire_cat02_tipo_comprobante;
pub mod sire_cat03_moneda;
pub mod sire_cat11_tipo_afectacion_igv;
pub mod sire_cat_estados;

pub use sire_cat01_tipo_doc_identidad::SireCatalogo01TipoDocumentoIdentidad;
pub use sire_cat02_tipo_comprobante::SireCatalogo02TipoComprobante;
pub use sire_cat03_moneda::SireCatalogo03Moneda;
pub use sire_cat11_tipo_afectacion_igv::SireCatalogo11TipoAfectacionIgv;
pub use sire_cat_estados::{
    SireEstadoComprobantePropuesta, SireEstadoTicket, SireTipoProceso,
};
