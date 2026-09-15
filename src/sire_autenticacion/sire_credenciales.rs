// Copyright (c) 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Credenciales de Acceso a SUNAT SIRE
//!
//! Encapsula los identificadores de aplicación (Client ID / Client Secret) y las
//! credenciales de la Clave SOL exigidas por el flujo OAuth 2.0 de SUNAT.

use crate::sire_errores::{SireError, SireResultado};
use serde::{Deserialize, Serialize};

/// Estructura de credenciales requerida para la autenticación en el SIRE.
#[derive(Clone, Serialize, Deserialize)]
pub struct SireCredenciales {
    /// Client ID otorgado al registrar la aplicación en SUNAT Operaciones en Línea.
    pub client_id: String,

    /// Client Secret correspondiente a la aplicación registrada.
    pub client_secret: String,

    /// Número de Registro Único de Contribuyentes (RUC) de 11 dígitos.
    pub ruc: String,

    /// Nombre de usuario del operador o usuario secundario de la Clave SOL.
    pub usuario_sol: String,

    /// Contraseña asociada al usuario SOL.
    pub clave_sol: String,
}

impl SireCredenciales {
    /// Crea un nuevo conjunto de credenciales validando los campos básicos.
    ///
    /// # Errores
    /// Retorna `SireError::Validacion` si algún campo está vacío o si el RUC no cumple con el formato de 11 dígitos.
    pub fn nueva(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        ruc: impl Into<String>,
        usuario_sol: impl Into<String>,
        clave_sol: impl Into<String>,
    ) -> SireResultado<Self> {
        let credenciales = Self {
            client_id: client_id.into().trim().to_string(),
            client_secret: client_secret.into().trim().to_string(),
            ruc: ruc.into().trim().to_string(),
            usuario_sol: usuario_sol.into().trim().to_string(),
            clave_sol: clave_sol.into().trim().to_string(),
        };

        credenciales.validar()?;
        Ok(credenciales)
    }

    /// Valida la consistencia estructural de las credenciales antes de enviar peticiones.
    pub fn validar(&self) -> SireResultado<()> {
        if self.client_id.is_empty() {
            return Err(SireError::Validacion("El client_id no puede estar vacío".into()));
        }
        if self.client_secret.is_empty() {
            return Err(SireError::Validacion("El client_secret no puede estar vacío".into()));
        }
        if self.ruc.len() != 11 || !self.ruc.chars().all(|c| c.is_ascii_digit()) {
            return Err(SireError::Validacion(format!(
                "El RUC debe tener exactamente 11 dígitos numéricos, recibido: '{}'",
                self.ruc
            )));
        }
        if self.usuario_sol.is_empty() {
            return Err(SireError::Validacion("El usuario_sol no puede estar vacío".into()));
        }
        if self.clave_sol.is_empty() {
            return Err(SireError::Validacion("La clave_sol no puede estar vacía".into()));
        }
        Ok(())
    }

    /// Genera el valor combinado `{RUC}{USUARIO_SOL}` requerido en el campo `username` del OAuth 2.0.
    pub fn username_compuesto(&self) -> String {
        format!("{}{}", self.ruc, self.usuario_sol)
    }
}

// Implementación de Debug personalizada para no exponer la clave SOL ni el secret en logs.
impl std::fmt::Debug for SireCredenciales {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SireCredenciales")
            .field("client_id", &self.client_id)
            .field("client_secret", &"[PROTEGIDO]")
            .field("ruc", &self.ruc)
            .field("usuario_sol", &self.usuario_sol)
            .field("clave_sol", &"[PROTEGIDO]")
            .finish()
    }
}
