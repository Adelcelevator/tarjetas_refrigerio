use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Roles{
    RegistrarComprobante,
    DebitoTarjeta,
    RegistroTarjeta,
    AprobacionComprobantes
}

impl fmt::Display for Roles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Roles::RegistrarComprobante => write!(f,"REGISTRO_COMPROBANTE"),
            Roles::DebitoTarjeta => write!(f,"DEBITO_TARJETA"),
            Roles::RegistroTarjeta => write!(f,"REGISTRO_TARJETA"),
            Roles::AprobacionComprobantes=>write!(f,"APROBACION_COMPROBANTES"),
        }
    }
}