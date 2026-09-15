// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Modelos del Registro de Ventas e Ingresos Electrónico (RVIE)
//!
//! Representación fuertemente tipada de los comprobantes de venta con importes
//! monetarios en `rust_decimal::Decimal` (precisión exacta, cero coma flotante).

use crate::sire_catalogos::{
    SireCatalogo01TipoDocumentoIdentidad, SireCatalogo02TipoComprobante, SireCatalogo03Moneda,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Comprobante de venta o ingreso para el RVIE según el Anexo de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SireComprobanteVenta {
    /// Periodo tributario en formato YYYYMM (ej. "202609").
    pub periodo: String,

    /// Código de Anotación de Registro (CAR) generado por SUNAT o el contribuyente.
    pub car: Option<String>,

    /// Fecha de emisión del comprobante (formato YYYY-MM-DD o DD/MM/YYYY).
    pub fecha_emision: String,

    /// Fecha de vencimiento o pago (obligatorio en servicios públicos).
    pub fecha_vencimiento: Option<String>,

    /// Tipo de comprobante según Catálogo 02 de SUNAT.
    pub tipo_comprobante: SireCatalogo02TipoComprobante,

    /// Serie del comprobante (ej. "F001", "B001", "0001").
    pub serie: String,

    /// Número correlativo del comprobante.
    pub numero: String,

    /// Tipo de documento de identidad del cliente según Catálogo 01.
    pub tipo_doc_cliente: SireCatalogo01TipoDocumentoIdentidad,

    /// Número de documento de identidad del cliente (RUC, DNI, etc.).
    pub num_doc_cliente: String,

    /// Razón social, apellidos y nombres del cliente.
    pub razon_social_cliente: String,

    /// Valor facturado de la exportación (si aplica).
    pub valor_exportacion: Decimal,

    /// Base imponible de la operación gravada con IGV.
    pub base_imponible_gravada: Decimal,

    /// Descuento a la base imponible.
    pub descuento_base_imponible: Decimal,

    /// Importe del Impuesto General a las Ventas (IGV / IPM).
    pub monto_igv: Decimal,

    /// Descuento al IGV.
    pub descuento_igv: Decimal,

    /// Importe total de la operación exonerada de IGV.
    pub monto_exonerado: Decimal,

    /// Importe total de la operación inafecta de IGV.
    pub monto_inafecto: Decimal,

    /// Importe del Impuesto Selectivo al Consumo (ISC).
    pub monto_isc: Decimal,

    /// Base imponible del arroz pilado (IVAP).
    pub base_imponible_ivap: Decimal,

    /// Importe del IVAP.
    pub monto_ivap: Decimal,

    /// Impuesto al Consumo de las Bolsas de Plástico (ICBPER).
    pub monto_icbper: Decimal,

    /// Otros cargos o tributos no comprendidos en la base imponible.
    pub otros_tributos: Decimal,

    /// Importe total del comprobante de venta.
    pub importe_total: Decimal,

    /// Código de moneda de emisión según Catálogo 03.
    pub moneda: SireCatalogo03Moneda,

    /// Tipo de cambio oficial con 3 decimales (si la moneda es extranjera).
    pub tipo_cambio: Option<Decimal>,

    /// Tipo de comprobante referenciado (obligatorio en Notas de Crédito / Débito).
    pub tipo_comprobante_modificado: Option<SireCatalogo02TipoComprobante>,

    /// Serie del comprobante referenciado.
    pub serie_modificada: Option<String>,

    /// Número del comprobante referenciado.
    pub numero_modificado: Option<String>,
}

impl SireComprobanteVenta {
    /// Genera la línea formateada del comprobante delimitada por el separador tubería `|`
    /// conforme a la especificación oficial del archivo plano de reemplazo de RVIE de SUNAT.
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

        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|{}|{}|{}|{}|{}|",
            self.periodo,
            car_val,
            self.fecha_emision,
            fec_venc,
            self.tipo_comprobante.codigo(),
            self.serie,
            self.numero,
            self.tipo_doc_cliente.codigo(),
            self.num_doc_cliente,
            self.razon_social_cliente,
            self.valor_exportacion,
            self.base_imponible_gravada,
            self.descuento_base_imponible,
            self.monto_igv,
            self.descuento_igv,
            self.monto_exonerado,
            self.monto_inafecto,
            self.monto_isc,
            self.base_imponible_ivap,
            self.monto_ivap,
            self.monto_icbper,
            self.otros_tributos,
            self.importe_total,
            self.moneda.codigo(),
            tc,
            cod_mod,
            serie_mod,
            num_mod
        )
    }
}
