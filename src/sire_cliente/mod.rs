// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo Cliente HTTP de SUNAT SIRE (`sire_cliente`)
//!
//! Contiene el cliente asíncrono sobre Reqwest/Tokio para el consumo de todos los
//! endpoints oficiales del SIRE.

pub mod sire_cliente_http;
pub mod sire_configuracion;

pub use sire_cliente_http::SireCliente;
pub use sire_configuracion::SireConfiguracion;
