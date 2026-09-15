// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # sunat_sire
//!
//! Cliente y utilidades de alto rendimiento para la integración con los servicios web
//! del **Sistema Integrado de Registros Electrónicos (SIRE)** de la **SUNAT** (Perú).
//!
//! Proporciona soporte integral tanto para la **generación e inspección offline** de archivos
//! planos (`.txt`/`.zip` con hash SHA-256) como para la **interacción en línea con el API REST**
//! oficial (autenticación OAuth 2.0 Clave SOL, consultas, propuestas y sondeo de tickets).
//!
//! ## Estructura de Módulos
//!
//! - [`sire_autenticacion`]: Gestión de credenciales Clave SOL, ambientes y tokens Bearer en memoria con auto-refresco asíncrono.
//! - [`sire_cliente`]: Cliente HTTP asíncrono sobre Reqwest/Tokio con inyección de cabeceras y control de reintentos.
//! - [`sire_catalogos`]: Catálogos normalizados de SUNAT (tipos de documentos, comprobantes, monedas, afectaciones IGV y estados).
//! - [`sire_rvie`]: Registro de Ventas e Ingresos Electrónico (modelos con `rust_decimal::Decimal`, generador plano, empaquetador ZIP y API).
//! - [`sire_rce`]: Registro de Compras Electrónico (compras nacionales, casillas de crédito fiscal en `Decimal` y servicios API).
//! - [`sire_tickets`]: Monitoreo de procesos en segundo plano de SUNAT mediante sondeos asíncronos (*polling*) no bloqueantes.
//! - [`sire_tus`]: Cargas masivas y resumibles de archivos mediante el protocolo abierto TUS 1.0.0.
//! - [`sire_errores`]: Jerarquía fuertemente tipada de errores basada en `thiserror`.

#![warn(missing_docs)]

pub mod sire_autenticacion;
pub mod sire_catalogos;
pub mod sire_cliente;
pub mod sire_errores;
pub mod sire_rce;
pub mod sire_rvie;
pub mod sire_tickets;
pub mod sire_tus;

// Re-exportaciones de conveniencia para la raíz del crate
pub use sire_autenticacion::{SireAmbiente, SireCredenciales, SireGestorToken, SireToken};
pub use sire_cliente::{SireCliente, SireConfiguracion};
pub use sire_errores::{SireError, SireResultado};
pub use sire_rce::{
    sire_empaquetar_zip_rce, sire_generar_archivo_plano_rce, sire_reemplazar_propuesta_rce_tus,
    SireComprobanteCompra,
};
pub use sire_rvie::{
    sire_empaquetar_zip_rvie, sire_generar_archivo_plano_rvie, sire_reemplazar_propuesta_rvie_tus,
    SireComprobanteVenta,
};
pub use sire_tickets::{SireArchivoRespuesta, SireTicket, sire_consultar_ticket, sire_esperar_ticket};
pub use sire_tus::{
    SireClienteTus, SireConfiguracionTus, SireMetadatosTus, SireProgresoTus, SireRespuestaTus,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn rvie_debe_generar_archivo_plano_y_zip_con_sha256_valido() {
        let comprobante = SireComprobanteVenta {
            periodo: "202609".to_string(),
            car: Some("2060000000101F00100000001".to_string()),
            fecha_emision: "2026-09-01".to_string(),
            fecha_vencimiento: None,
            tipo_comprobante: sire_catalogos::SireCatalogo02TipoComprobante::Factura,
            serie: "F001".to_string(),
            numero: "00000001".to_string(),
            tipo_doc_cliente: sire_catalogos::SireCatalogo01TipoDocumentoIdentidad::Ruc,
            num_doc_cliente: "20123456789".to_string(),
            razon_social_cliente: "EMPRESA CLIENTE S.A.C.".to_string(),
            valor_exportacion: dec!(0.00),
            base_imponible_gravada: dec!(1000.00),
            descuento_base_imponible: dec!(0.00),
            monto_igv: dec!(180.00),
            descuento_igv: dec!(0.00),
            monto_exonerado: dec!(0.00),
            monto_inafecto: dec!(0.00),
            monto_isc: dec!(0.00),
            base_imponible_ivap: dec!(0.00),
            monto_ivap: dec!(0.00),
            monto_icbper: dec!(0.00),
            otros_tributos: dec!(0.00),
            importe_total: dec!(1180.00),
            moneda: sire_catalogos::SireCatalogo03Moneda::Pen,
            tipo_cambio: None,
            tipo_comprobante_modificado: None,
            serie_modificada: None,
            numero_modificado: None,
        };

        let comprobantes = vec![comprobante];
        let plano = sire_generar_archivo_plano_rvie(&comprobantes);
        assert!(plano.contains("2060000000101F00100000001"));
        assert!(plano.contains("|1000.00|0.00|180.00|"));
        assert!(plano.contains("|1180.00|PEN|"));

        let (nombre_zip, bytes_zip, hash_sha256) =
            sire_empaquetar_zip_rvie("20600000001", "202609", &plano).expect("Error al empaquetar ZIP");

        assert!(nombre_zip.starts_with("LE2060000000120260900140400021112.zip"));
        assert!(!bytes_zip.is_empty());
        assert_eq!(hash_sha256.len(), 64); // SHA-256 son 64 caracteres hexadecimales
    }

    #[test]
    fn rce_debe_generar_archivo_plano_y_zip_con_sha256_valido() {
        let compra = SireComprobanteCompra {
            periodo: "202609".to_string(),
            car: Some("2012345678901F00100000050".to_string()),
            fecha_emision: "2026-09-02".to_string(),
            fecha_vencimiento: None,
            tipo_comprobante: sire_catalogos::SireCatalogo02TipoComprobante::Factura,
            serie: "F001".to_string(),
            numero: "00000050".to_string(),
            tipo_doc_proveedor: sire_catalogos::SireCatalogo01TipoDocumentoIdentidad::Ruc,
            num_doc_proveedor: "20987654321".to_string(),
            razon_social_proveedor: "PROVEEDOR INDUSTRIAL S.A.C.".to_string(),
            bi_gravada_dg: dec!(500.00),
            igv_dg: dec!(90.00),
            bi_gravada_dng: dec!(0.00),
            igv_dng: dec!(0.00),
            bi_gravada_dsg: dec!(0.00),
            igv_dsg: dec!(0.00),
            adquisiciones_no_gravadas: dec!(0.00),
            monto_isc: dec!(0.00),
            monto_icbper: dec!(0.00),
            otros_tributos: dec!(0.00),
            importe_total: dec!(590.00),
            moneda: sire_catalogos::SireCatalogo03Moneda::Pen,
            tipo_cambio: None,
            tipo_comprobante_modificado: None,
            serie_modificada: None,
            numero_modificado: None,
            constancia_detraccion: None,
            fecha_detraccion: None,
        };

        let compras = vec![compra];
        let plano = sire_generar_archivo_plano_rce(&compras);
        assert!(plano.contains("2012345678901F00100000050"));
        assert!(plano.contains("|500.00|90.00|"));
        assert!(plano.contains("|590.00|PEN|"));

        let (nombre_zip, bytes_zip, hash_sha256) =
            sire_empaquetar_zip_rce("20600000001", "202609", &plano).expect("Error al empaquetar ZIP");

        assert!(nombre_zip.starts_with("LE2060000000120260900080400021112.zip"));
        assert!(!bytes_zip.is_empty());
        assert_eq!(hash_sha256.len(), 64);
    }

    #[test]
    fn tus_debe_formatear_metadatos_y_calcular_progreso() {
        let metadatos = SireMetadatosTus::nuevo(
            "LE2060000000120260900140400021112.zip",
            "a5b42d07ebc6c885e9270ed5bd3b8254cb6d13804837fd0f59ad4efa1c4fc35c",
            "20600000001",
            "202609",
        );

        let cabecera = metadatos.a_cabecera_upload_metadata();
        assert!(cabecera.contains("filename "));
        assert!(cabecera.contains("hash "));
        assert!(cabecera.contains("numRuc "));
        assert!(cabecera.contains("perTributario "));

        let config = SireConfiguracionTus::nuevo().con_tamano_chunk_mb(5);
        assert_eq!(config.tamano_chunk_bytes, 5 * 1024 * 1024);

        let progreso = SireProgresoTus::calcular(5 * 1024 * 1024, 10 * 1024 * 1024, 1, 2);
        assert_eq!(progreso.porcentaje, 50.0);
        assert_eq!(progreso.fragmento_actual, 1);
        assert_eq!(progreso.total_fragmentos, 2);
    }
}

