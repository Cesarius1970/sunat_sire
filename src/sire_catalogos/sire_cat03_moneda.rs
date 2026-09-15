// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Catálogo 03 de SUNAT: Tipo de Moneda (ISO 4217)
//!
//! Especifica los códigos de divisa oficiales permitidos por SUNAT en los
//! registros electrónicos RVIE y RCE del SIRE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos oficiales del Catálogo 03 de SUNAT para divisas (ISO 4217).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireCatalogo03Moneda {
    /// Sol peruano (PEN).
    #[default]
    #[serde(rename = "PEN")]
    Pen,

    /// Dólar estadounidense (USD).
    #[serde(rename = "USD")]
    Usd,

    /// Euro (EUR).
    #[serde(rename = "EUR")]
    Eur,

    /// Libra esterlina (GBP).
    #[serde(rename = "GBP")]
    Gbp,

    /// Yen japonés (JPY).
    #[serde(rename = "JPY")]
    Jpy,

    /// Franco suizo (CHF).
    #[serde(rename = "CHF")]
    Chf,

    /// Dólar canadiense (CAD).
    #[serde(rename = "CAD")]
    Cad,

    /// Peso chileno (CLP).
    #[serde(rename = "CLP")]
    Clp,

    /// Peso colombiano (COP).
    #[serde(rename = "COP")]
    Cop,

    /// Real brasileño (BRL).
    #[serde(rename = "BRL")]
    Brl,

    /// Yuan chino (CNY).
    #[serde(rename = "CNY")]
    Cny,
}

impl SireCatalogo03Moneda {
    /// Retorna el código alfabético ISO 4217 exigido por SUNAT.
    pub const fn codigo(&self) -> &'static str {
        match self {
            Self::Pen => "PEN",
            Self::Usd => "USD",
            Self::Eur => "EUR",
            Self::Gbp => "GBP",
            Self::Jpy => "JPY",
            Self::Chf => "CHF",
            Self::Cad => "CAD",
            Self::Clp => "CLP",
            Self::Cop => "COP",
            Self::Brl => "BRL",
            Self::Cny => "CNY",
        }
    }

    /// Retorna el nombre en español de la moneda.
    pub const fn descripcion(&self) -> &'static str {
        match self {
            Self::Pen => "Sol Peruano",
            Self::Usd => "Dólar Estadounidense",
            Self::Eur => "Euro",
            Self::Gbp => "Libra Esterlina",
            Self::Jpy => "Yen Japonés",
            Self::Chf => "Franco Suizo",
            Self::Cad => "Dólar Canadiense",
            Self::Clp => "Peso Chileno",
            Self::Cop => "Peso Colombiano",
            Self::Brl => "Real Brasileño",
            Self::Cny => "Yuan Chino",
        }
    }

    /// Parsea una cadena de texto (ej. "PEN" o "USD") a la variante correspondiente.
    pub fn desde_codigo(codigo: &str) -> Option<Self> {
        match codigo.trim().to_uppercase().as_str() {
            "PEN" => Some(Self::Pen),
            "USD" => Some(Self::Usd),
            "EUR" => Some(Self::Eur),
            "GBP" => Some(Self::Gbp),
            "JPY" => Some(Self::Jpy),
            "CHF" => Some(Self::Chf),
            "CAD" => Some(Self::Cad),
            "CLP" => Some(Self::Clp),
            "COP" => Some(Self::Cop),
            "BRL" => Some(Self::Brl),
            "CNY" => Some(Self::Cny),
            _ => None,
        }
    }

    /// Indica si la divisa es la moneda nacional (Soles).
    pub const fn es_moneda_nacional(&self) -> bool {
        matches!(self, Self::Pen)
    }
}

impl fmt::Display for SireCatalogo03Moneda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
