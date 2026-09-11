// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Catálogo 02 de SUNAT: Tipo de Comprobante de Pago o Documento
//!
//! Especifica los tipos de comprobantes de pago, notas de crédito/débito y otros
//! documentos admitidos en las propuestas y registros de RVIE y RCE del SIRE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos oficiales del Catálogo 02 de SUNAT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SireCatalogo02TipoComprobante {
    /// Factura electrónica o física.
    #[default]
    #[serde(rename = "01")]
    Factura,

    /// Recibo por Honorarios.
    #[serde(rename = "02")]
    ReciboPorHonorarios,

    /// Boleta de Venta.
    #[serde(rename = "03")]
    BoletaDeVenta,

    /// Liquidación de Compra.
    #[serde(rename = "04")]
    LiquidacionDeCompra,

    /// Boleto de Transporte Aéreo.
    #[serde(rename = "05")]
    BoletoTransporteAereo,

    /// Carta de Porte Aéreo.
    #[serde(rename = "06")]
    CartaPorteAereo,

    /// Nota de Crédito.
    #[serde(rename = "07")]
    NotaDeCredito,

    /// Nota de Débito.
    #[serde(rename = "08")]
    NotaDeDebito,

    /// Guía de Remisión - Remitente.
    #[serde(rename = "09")]
    GuiaRemisionRemitente,

    /// Recibo por Arrendamiento.
    #[serde(rename = "11")]
    ReciboPorArrendamiento,

    /// Ticket o cinta emitido por máquina registradora.
    #[serde(rename = "12")]
    TicketMaquinaRegistradora,

    /// Documento emitido por bancos, instituciones financieras y crediticias.
    #[serde(rename = "13")]
    DocumentoBancario,

    /// Recibo por servicios públicos (luz, agua, telecomunicaciones).
    #[serde(rename = "14")]
    ReciboServiciosPublicos,

    /// Boleto emitido por las empresas de transporte público urbano.
    #[serde(rename = "15")]
    BoletoTransporteUrbano,

    /// Boleto de viaje emitido por empresas de transporte interprovincial.
    #[serde(rename = "16")]
    BoletoTransporteInterprovincial,

    /// Documento emitido por la Iglesia Católica por arrendamiento.
    #[serde(rename = "18")]
    DocumentoIglesiaCatolica,

    /// Póliza de Adjudicación emitida por martilleros y comisionistas.
    #[serde(rename = "23")]
    PolizaAdjudicacion,

    /// Guía de Remisión - Transportista.
    #[serde(rename = "31")]
    GuiaRemisionTransportista,

    /// Comprobante de Retención.
    #[serde(rename = "20")]
    ComprobanteRetencion,

    /// Comprobante de Percepción.
    #[serde(rename = "40")]
    ComprobantePercepcion,

    /// Declaración Única de Aduanas (DUA) / Declaración Aduanera de Mercancías (DAM).
    #[serde(rename = "50")]
    DuaDam,

    /// Declaración Simplificada de Importación (DSI).
    #[serde(rename = "52")]
    DeclaracionSimplificadaImportacion,

    /// Comprobante emitido por no domiciliado (RCE no domiciliados).
    #[serde(rename = "91")]
    ComprobanteNoDomiciliado,

    /// Nota de Crédito emitida por no domiciliado.
    #[serde(rename = "97")]
    NotaCreditoNoDomiciliado,

    /// Nota de Débito emitida por no domiciliado.
    #[serde(rename = "98")]
    NotaDebitoNoDomiciliado,

    /// Otros documentos autorizados no catalogados específicamente.
    #[serde(rename = "00")]
    Otros,
}

impl SireCatalogo02TipoComprobante {
    /// Retorna el código de dos dígitos de SUNAT.
    pub const fn codigo(&self) -> &'static str {
        match self {
            Self::Factura => "01",
            Self::ReciboPorHonorarios => "02",
            Self::BoletaDeVenta => "03",
            Self::LiquidacionDeCompra => "04",
            Self::BoletoTransporteAereo => "05",
            Self::CartaPorteAereo => "06",
            Self::NotaDeCredito => "07",
            Self::NotaDeDebito => "08",
            Self::GuiaRemisionRemitente => "09",
            Self::ReciboPorArrendamiento => "11",
            Self::TicketMaquinaRegistradora => "12",
            Self::DocumentoBancario => "13",
            Self::ReciboServiciosPublicos => "14",
            Self::BoletoTransporteUrbano => "15",
            Self::BoletoTransporteInterprovincial => "16",
            Self::DocumentoIglesiaCatolica => "18",
            Self::ComprobanteRetencion => "20",
            Self::PolizaAdjudicacion => "23",
            Self::GuiaRemisionTransportista => "31",
            Self::ComprobantePercepcion => "40",
            Self::DuaDam => "50",
            Self::DeclaracionSimplificadaImportacion => "52",
            Self::ComprobanteNoDomiciliado => "91",
            Self::NotaCreditoNoDomiciliado => "97",
            Self::NotaDebitoNoDomiciliado => "98",
            Self::Otros => "00",
        }
    }

    /// Retorna la descripción oficial en español del tipo de comprobante.
    pub const fn descripcion(&self) -> &'static str {
        match self {
            Self::Factura => "Factura",
            Self::ReciboPorHonorarios => "Recibo por Honorarios",
            Self::BoletaDeVenta => "Boleta de Venta",
            Self::LiquidacionDeCompra => "Liquidación de Compra",
            Self::BoletoTransporteAereo => "Boleto de Transporte Aéreo",
            Self::CartaPorteAereo => "Carta de Porte Aéreo",
            Self::NotaDeCredito => "Nota de Crédito",
            Self::NotaDeDebito => "Nota de Débito",
            Self::GuiaRemisionRemitente => "Guía de Remisión - Remitente",
            Self::ReciboPorArrendamiento => "Recibo por Arrendamiento",
            Self::TicketMaquinaRegistradora => "Ticket o Cinta de Máquina Registradora",
            Self::DocumentoBancario => "Documento emitido por Bancos y Entidades Financieras",
            Self::ReciboServiciosPublicos => "Recibo por Servicios Públicos",
            Self::BoletoTransporteUrbano => "Boleto de Transporte Urbano",
            Self::BoletoTransporteInterprovincial => "Boleto de Transporte Terrestre Interprovincial",
            Self::DocumentoIglesiaCatolica => "Documento Iglesia Católica",
            Self::ComprobanteRetencion => "Comprobante de Retención",
            Self::PolizaAdjudicacion => "Póliza de Adjudicación",
            Self::GuiaRemisionTransportista => "Guía de Remisión - Transportista",
            Self::ComprobantePercepcion => "Comprobante de Percepción",
            Self::DuaDam => "Declaración Única de Aduanas (DUA / DAM)",
            Self::DeclaracionSimplificadaImportacion => "Declaración Simplificada de Importación (DSI)",
            Self::ComprobanteNoDomiciliado => "Comprobante emitido por Sujeto No Domiciliado",
            Self::NotaCreditoNoDomiciliado => "Nota de Crédito - No Domiciliado",
            Self::NotaDebitoNoDomiciliado => "Nota de Débito - No Domiciliado",
            Self::Otros => "Otros Documentos Autorizados",
        }
    }

    /// Parsea un código de texto al tipo enumerado.
    pub fn desde_codigo(codigo: &str) -> Option<Self> {
        match codigo.trim() {
            "01" => Some(Self::Factura),
            "02" => Some(Self::ReciboPorHonorarios),
            "03" => Some(Self::BoletaDeVenta),
            "04" => Some(Self::LiquidacionDeCompra),
            "05" => Some(Self::BoletoTransporteAereo),
            "06" => Some(Self::CartaPorteAereo),
            "07" => Some(Self::NotaDeCredito),
            "08" => Some(Self::NotaDeDebito),
            "09" => Some(Self::GuiaRemisionRemitente),
            "11" => Some(Self::ReciboPorArrendamiento),
            "12" => Some(Self::TicketMaquinaRegistradora),
            "13" => Some(Self::DocumentoBancario),
            "14" => Some(Self::ReciboServiciosPublicos),
            "15" => Some(Self::BoletoTransporteUrbano),
            "16" => Some(Self::BoletoTransporteInterprovincial),
            "18" => Some(Self::DocumentoIglesiaCatolica),
            "20" => Some(Self::ComprobanteRetencion),
            "23" => Some(Self::PolizaAdjudicacion),
            "31" => Some(Self::GuiaRemisionTransportista),
            "40" => Some(Self::ComprobantePercepcion),
            "50" => Some(Self::DuaDam),
            "52" => Some(Self::DeclaracionSimplificadaImportacion),
            "91" => Some(Self::ComprobanteNoDomiciliado),
            "97" => Some(Self::NotaCreditoNoDomiciliado),
            "98" => Some(Self::NotaDebitoNoDomiciliado),
            "00" => Some(Self::Otros),
            _ => None,
        }
    }
}

impl fmt::Display for SireCatalogo02TipoComprobante {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
