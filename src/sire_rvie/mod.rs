// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo del Registro de Ventas e Ingresos Electrónico (`sire_rvie`)
//!
//! Contiene los modelos, generadores de archivos planos con validación SHA-256
//! y clientes de integración con los servicios de RVIE de SUNAT.

pub mod sire_generador_rvie;
pub mod sire_modelo_rvie;
pub mod sire_propuesta_rvie;

pub use sire_generador_rvie::{sire_empaquetar_zip_rvie, sire_generar_archivo_plano_rvie};
pub use sire_modelo_rvie::SireComprobanteVenta;
pub use sire_propuesta_rvie::{
    sire_aceptar_propuesta_rvie, sire_consultar_propuesta_rvie, sire_reemplazar_propuesta_rvie,
    SireRespuestaPropuestaRvie,
};
