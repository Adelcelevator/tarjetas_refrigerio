use serde::{Deserialize, Serialize};

use crate::models::data_model::tarjeta_model::Tarjeta;


#[derive( Serialize, Deserialize,Debug,Clone)]
pub struct LoginResponse{
    pub token:String,
    pub nombre:String,
    pub roles:Vec<Option<String>>,
    pub representados: Option<Vec<RepresentadosResponse>>,
    pub tarjetas: Option<Vec<Tarjeta>>,
}

#[derive( Serialize, Deserialize,Debug,Clone)]
pub struct RepresentadosResponse{
    pub id: i32,
    pub nombre: String,
}