use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Estados{
    Activo,
    Inactivo,
    Eliminado,
    CambiarClave,
    PorAutorizar,
    Autorizado,
    Anulado,
}

impl fmt::Display for Estados {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Estados::Activo => write!(f,"Activo"),
            Estados::Eliminado => write!(f,"Eliminado"),
            Estados::Inactivo => write!(f,"Inactivo"),
            Estados::CambiarClave=>write!(f,"PassChange"),
            Estados::PorAutorizar=>write!(f,"Por Autorizar"),
            Estados::Autorizado=>write!(f,"Autorizado"),
            Estados::Anulado=>write!(f,"Anulado"),
        }
    }
}