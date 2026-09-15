// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Módulo de Gestión de Tickets Asíncronos (`sire_tickets`)
//!
//! Monitorea el ciclo de vida de los procesos en segundo plano de SUNAT SIRE y
//! gestiona la descarga de archivos generados.

pub mod sire_modelo_ticket;
pub mod sire_polling_ticket;

pub use sire_modelo_ticket::{SireArchivoRespuesta, SireTicket};
pub use sire_polling_ticket::{sire_consultar_ticket, sire_esperar_ticket};
