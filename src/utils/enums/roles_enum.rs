use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum _Roles{
    RegistrarComprobante,
    DebitoTarjeta,
    RegistroTarjeta,
    AprobacionComprobantes
}

impl fmt::Display for _Roles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _Roles::RegistrarComprobante => write!(f,"REGISTRO_COMPROBANTE"),
            _Roles::DebitoTarjeta => write!(f,"DEBITO_TARJETA"),
            _Roles::RegistroTarjeta => write!(f,"REGISTRO_TARJETA"),
            _Roles::AprobacionComprobantes=>write!(f,"APROBACION_COMPROBANTES"),
        }
    }
}