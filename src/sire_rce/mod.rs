// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo del Registro de Compras Electrónico (`sire_rce`)
//!
//! Modelos, generadores de archivos planos con validación SHA-256
//! y clientes de integración con los servicios de RCE de SUNAT.

pub mod sire_generador_rce;
pub mod sire_modelo_rce;
pub mod sire_propuesta_rce;

pub use sire_generador_rce::{sire_empaquetar_zip_rce, sire_generar_archivo_plano_rce};
pub use sire_modelo_rce::SireComprobanteCompra;
pub use sire_propuesta_rce::{
    sire_aceptar_propuesta_rce, sire_consultar_propuesta_rce, sire_reemplazar_propuesta_rce,
    SireRespuestaPropuestaRce,
};
