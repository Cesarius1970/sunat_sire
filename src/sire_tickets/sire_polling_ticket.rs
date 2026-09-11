// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Consulta Asíncrona y Polling de Tickets en SUNAT SIRE
//!
//! Implementa la consulta individual y el sondeo continuo no bloqueante (*polling*)
//! mediante `tokio::time::sleep`, con retroceso y límite de intentos configurables.

use std::time::Duration;
use tracing::{debug, info, warn};

use crate::sire_cliente::SireCliente;
use crate::sire_errores::{SireError, SireResultado};
use crate::sire_tickets::sire_modelo_ticket::SireTicket;

/// Consulta el estado actual de un ticket en la API del SIRE.
pub async fn sire_consultar_ticket(
    cliente: &SireCliente,
    num_ticket: &str,
) -> SireResultado<SireTicket> {
    let ruta = format!(
        "/v1/contribuyente/migeigv/libros/rvierce/gestionprocesos/web/procesos/consulta/proceso?numTicket={}",
        num_ticket.trim()
    );

    let respuesta = cliente.ejecutar_get(&ruta, None).await?;
    let ticket: SireTicket = respuesta.json().await.map_err(SireError::Red)?;
    Ok(ticket)
}

/// Espera de manera asíncrona la finalización de un ticket realizando sondeos periódicos no bloqueantes.
///
/// # Parámetros
/// - `cliente`: Referencia al cliente HTTP autenticado.
/// - `num_ticket`: Número de ticket asignado por SUNAT.
/// - `intervalo`: Tiempo de espera entre cada consulta consecutiva.
/// - `intentos_maximos`: Límite máximo de verificaciones antes de abortar por tiempo excedido.
pub async fn sire_esperar_ticket(
    cliente: &SireCliente,
    num_ticket: &str,
    intervalo: Duration,
    intentos_maximos: u32,
) -> SireResultado<SireTicket> {
    info!(ticket = %num_ticket, "Iniciando sondeo asíncrono de ticket en SUNAT");

    for intento in 1..=intentos_maximos {
        let ticket = sire_consultar_ticket(cliente, num_ticket).await?;

        if ticket.esta_completado() {
            info!(
                ticket = %num_ticket,
                estado = ?ticket.estado_tipado(),
                intentos = intento,
                "Ticket finalizado exitosamente en SUNAT"
            );
            return Ok(ticket);
        }

        debug!(
            ticket = %num_ticket,
            intento = intento,
            max_intentos = intentos_maximos,
            "Ticket aún en proceso. Esperando intervalo..."
        );

        tokio::time::sleep(intervalo).await;
    }

    warn!(ticket = %num_ticket, "Tiempo de espera agotado en sondeo de ticket");
    Err(SireError::Validacion(format!(
        "Se agotó el número máximo de intentos ({}) esperando la resolución del ticket {}",
        intentos_maximos, num_ticket
    )))
}
