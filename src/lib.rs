// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # sunat_sire
//!
//! Cliente y utilidades para la integración con el **Sistema Integrado de Registros
//! Electrónicos (SIRE)** de la **SUNAT** (Superintendencia Nacional de Aduanas y de
//! Administración Tributaria - Perú).
//!
//! ## Estructura general
//!
//! Esta librería proporciona clientes y estructuras fuertemente tipadas para interactuar
//! con los servicios web de SUNAT SIRE, incluyendo:
//! - Registro de Compras Electrónico (RCE).
//! - Registro de Ventas e Ingresos Electrónico (RVIE).

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
