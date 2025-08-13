use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_comprobantes;

use super::archivo_model::Archivo;

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_comprobantes )]
pub struct ComprobanteConsulta {
    pub comp_id: Option<i32>,
    pub comp_numero: String,
    pub per_id: Option<i32>,
    pub estado: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion: Option<String>,
    pub comp_valor: f64,
    pub comp_path_fisico: String,
}

impl ComprobanteConsulta {
    pub fn void_init() -> ComprobanteConsulta {
        ComprobanteConsulta{
            comp_id: None,
            comp_numero:String::new(),
            comp_valor: 0.0,
            per_id: None,
            estado:String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_creacion:String::new(),
            fe_modificacion:Some(Local::now().naive_local()),
            usr_modificacion:Some(String::new()),
            comp_path_fisico: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ComprobanteRespuesta {
    pub comp_numero: String,
    pub comp_valor:f64,
    pub estado:String,
    pub fe_creacion: NaiveDateTime,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion:String,
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ComprobanteNuevoReq{
    pub numero: String,
    pub valor:f64,
    pub usuario: String,
    pub file: Archivo,
    pub representados: Vec<i32>
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ComprobanteNuevo{
    pub numero: String,
    pub valor:f64,
    pub usuario: String,
    pub file: String,
    pub persona_id: i32
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct AutorizarComprobante{
    pub numeros: Vec<String>,
    pub usuario: String
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ComprobanteReq{
    pub autorizar: bool,
    pub usuario: String
}