// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo de Autenticación de SUNAT SIRE (`sire_autenticacion`)
//!
//! Implementa la seguridad, gestión de credenciales Clave SOL y administración de
//! tokens OAuth 2.0 para los diferentes ambientes de SUNAT (Producción, Beta y Personalizado).

pub mod sire_ambiente;
pub mod sire_credenciales;
pub mod sire_gestor_token;
pub mod sire_token;

pub use sire_ambiente::SireAmbiente;
pub use sire_credenciales::SireCredenciales;
pub use sire_gestor_token::SireGestorToken;
pub use sire_token::SireToken;
