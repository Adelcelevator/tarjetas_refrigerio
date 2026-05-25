use serde::{Deserialize, Serialize};

use crate::models::data_model::postgres::{tipo_pago_model::TipoPago, unidades_model::CargarUnidad};

#[derive( Serialize, Deserialize,Debug,Clone)]
pub struct TiposPagosUnidadesResponse{
    pub tipos_pagos:Vec<TipoPago>,
    pub unidades:Vec<CargarUnidad>,
}