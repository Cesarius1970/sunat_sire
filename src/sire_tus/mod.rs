// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo de Cargas Resumibles TUS (`sire_tus`)
//!
//! Implementación del protocolo abierto **TUS 1.0.0** para la carga masiva y reanudable
//! de archivos planos y comprimidos hacia los servicios web del SIRE de SUNAT.

pub mod sire_cliente_tus;
pub mod sire_configuracion_tus;
pub mod sire_metadatos_tus;
pub mod sire_progreso_tus;

pub use sire_cliente_tus::{SireClienteTus, SireRespuestaTus};
pub use sire_configuracion_tus::SireConfiguracionTus;
pub use sire_metadatos_tus::SireMetadatosTus;
pub use sire_progreso_tus::SireProgresoTus;
