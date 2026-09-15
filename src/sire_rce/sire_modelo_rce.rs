// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Modelos del Registro de Compras Electrónico (RCE)
//!
//! Estructuras tipadas para comprobantes de compras nacionales y no domiciliados,
//! con discriminación de casillas de crédito fiscal según la legislación tributaria peruana.
//! Todos los importes monetarios utilizan `rust_decimal::Decimal` (cero coma flotante).

use crate::sire_catalogos::{
    SireCatalogo01TipoDocumentoIdentidad, SireCatalogo02TipoComprobante, SireCatalogo03Moneda,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Comprobante de adquisición o compra para el RCE Nacional según Anexo de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SireComprobanteCompra {
    /// Periodo tributario en formato YYYYMM (ej. "202609").
    pub periodo: String,

    /// Código de Anotación de Registro (CAR) generado por SUNAT o el contribuyente.
    pub car: Option<String>,

    /// Fecha de emisión del comprobante (YYYY-MM-DD o DD/MM/YYYY).
    pub fecha_emision: String,

    /// Fecha de vencimiento o pago (si aplica).
    pub fecha_vencimiento: Option<String>,

    /// Tipo de comprobante según Catálogo 02.
    pub tipo_comprobante: SireCatalogo02TipoComprobante,

    /// Serie del comprobante.
    pub serie: String,

    /// Número correlativo o año de emisión de la DUA/DAM.
    pub numero: String,

    /// Tipo de documento de identidad del proveedor según Catálogo 01.
    pub tipo_doc_proveedor: SireCatalogo01TipoDocumentoIdentidad,

    /// Número de documento de identidad del proveedor (RUC, DNI, etc.).
    pub num_doc_proveedor: String,

    /// Razón social o nombres del proveedor.
    pub razon_social_proveedor: String,

    /// Base imponible de adquisiciones gravadas destinadas exclusivamente a operaciones gravadas y de exportación.
    pub bi_gravada_dg: Decimal,

    /// IGV correspondiente a adquisiciones destinadas a operaciones gravadas.
    pub igv_dg: Decimal,

    /// Base imponible de adquisiciones destinadas a operaciones gravadas y no gravadas (prorrata).
    pub bi_gravada_dng: Decimal,

    /// IGV correspondiente a operaciones con destino común (prorrata).
    pub igv_dng: Decimal,

    /// Base imponible de adquisiciones gravadas sin derecho a crédito fiscal.
    pub bi_gravada_dsg: Decimal,

    /// IGV de adquisiciones sin derecho a crédito fiscal.
    pub igv_dsg: Decimal,

    /// Valor de adquisiciones no gravadas (exoneradas o inafectas).
    pub adquisiciones_no_gravadas: Decimal,

    /// Impuesto Selectivo al Consumo (ISC).
    pub monto_isc: Decimal,

    /// Impuesto al Consumo de Bolsas de Plástico (ICBPER).
    pub monto_icbper: Decimal,

    /// Otros cargos y tributos que no forman parte de la base imponible.
    pub otros_tributos: Decimal,

    /// Importe total del comprobante de adquisición.
    pub importe_total: Decimal,

    /// Código de divisa según Catálogo 03.
    pub moneda: SireCatalogo03Moneda,

    /// Tipo de cambio oficial con 3 decimales (si la divisa no es PEN).
    pub tipo_cambio: Option<Decimal>,

    /// Tipo de comprobante modificado (en caso de Notas de Crédito / Débito).
    pub tipo_comprobante_modificado: Option<SireCatalogo02TipoComprobante>,

    /// Serie del comprobante referenciado.
    pub serie_modificada: Option<String>,

    /// Número del comprobante referenciado.
    pub numero_modificado: Option<String>,

    /// Número de constancia de depósito de detracción (si aplica).
    pub constancia_detraccion: Option<String>,

    /// Fecha de emisión de la constancia de detracción.
    pub fecha_detraccion: Option<String>,
}

impl SireComprobanteCompra {
    /// Formatea la línea de compra para el archivo plano del SIRE delimitada por `|`.
    pub fn formatear_linea_plano(&self) -> String {
        let tc = self
            .tipo_cambio
            .map(|t| format!("{:.3}", t))
            .unwrap_or_default();

        let cod_mod = self
            .tipo_comprobante_modificado
            .map(|t| t.codigo().to_string())
            .unwrap_or_default();

        let serie_mod = self.serie_modificada.as_deref().unwrap_or_default();
        let num_mod = self.numero_modificado.as_deref().unwrap_or_default();
        let car_val = self.car.as_deref().unwrap_or_default();
        let fec_venc = self.fecha_vencimiento.as_deref().unwrap_or_default();
        let const_det = self.constancia_detraccion.as_deref().unwrap_or_default();
        let fec_det = self.fecha_detraccion.as_deref().unwrap_or_default();

        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{}|{}|{}|{}|{}|{}|{}|",
            self.periodo,
            car_val,
            self.fecha_emision,
            fec_venc,
            self.tipo_comprobante.codigo(),
            self.serie,
            self.numero,
            self.tipo_doc_proveedor.codigo(),
            self.num_doc_proveedor,
            self.razon_social_proveedor,
            self.bi_gravada_dg,
            self.igv_dg,
            self.bi_gravada_dng,
            self.igv_dng,
            self.bi_gravada_dsg,
            self.igv_dsg,
            self.adquisiciones_no_gravadas,
            self.monto_isc,
            self.monto_icbper,
            self.otros_tributos,
            self.importe_total,
            self.moneda.codigo(),
            tc,
            cod_mod,
            serie_mod,
            num_mod,
            const_det,
            fec_det
        )
    }
}
