// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # sunat_sire
//!
//! Cliente y utilidades de alto rendimiento para la integración con los servicios web
//! del **Sistema Integrado de Registros Electrónicos (SIRE)** de la **SUNAT** (Perú).
//!
//! ## Arquitectura y Módulos Previstos
//!
//! La arquitectura de la librería está diseñada para garantizar seguridad de tipos en tiempo
//! de compilación, ejecución asíncrona no bloqueante (Tokio) y cero costos innecesarios de clonación:
//!
//! - **Autenticación (`auth`):** Gestión de credenciales Clave SOL y Client ID/Secret para la
//!   obtención y refresco de tokens OAuth 2.0 de SUNAT.
//! - **Cliente HTTP (`client`):** Cliente asíncrono con soporte de reintentos, backoff exponencial
//!   y trazabilidad estructurada (`tracing`).
//! - **Modelos de Dominio (`models`):** Estructuras para RCE (Registro de Compras Electrónico) y
//!   RVIE (Registro de Ventas e Ingresos Electrónico), esquemas de propuestas, tickets y resúmenes.
//! - **Manejo de Errores (`error`):** Jerarquía de errores tipados basada en `thiserror`.
//!
//! ## Directrices de Implementación
//!
//! - **Rendimiento:** Priorización de referencias prestadas (`&str`, `&[u8]`) sobre asignaciones en memoria.
//! - **Concurrencia:** Diseño asíncrono compatible con Tokio sin bloqueo de hilos de trabajo.
//! - **Estándares:** Apego riguroso a las especificaciones técnicas de los servicios API REST del SIRE.

#![warn(missing_docs)]

/// Función de verificación básica del crate.
///
/// # Parámetros
/// - `left`: Primer operando entero de 64 bits sin signo.
/// - `right`: Segundo operando entero de 64 bits sin signo.
///
/// # Retorno
/// Retorna la suma aritmética de ambos operandos.
///
/// # Ejemplos
/// ```
/// use sunat_sire::add;
///
/// let resultado = add(10, 20);
/// assert_eq!(resultado, 30);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    // Implementación aritmética directa libre de desbordamientos indeseados en casos estándar.
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_debe_retornar_suma_correcta_cuando_recibe_dos_numeros() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
