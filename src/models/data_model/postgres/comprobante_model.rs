use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};

use crate::{models::data_model::archivo_model::Archivo, repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_comprobantes};

#[derive(Queryable,Insertable,Serialize,Deserialize,Clone)]
#[diesel(table_name=tbl_comprobantes )]
pub struct InsertarComprobante {
    pub comp_id: Option<i32>,
    pub comp_numero: String,
    pub per_id: Option<i32>,
    pub estado: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion: Option<String>,
    pub comp_valor: rust_decimal::Decimal,
    pub comp_path_fisico: String,
}

#[derive(Queryable,Selectable,Serialize,Deserialize,Clone)]
#[diesel(table_name=tbl_comprobantes )]
pub struct ComprobanteConsulta{
    pub comp_id: Option<i32>,
    pub per_id: Option<i32>,
    pub comp_numero: String,
    pub estado: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub comp_valor: rust_decimal::Decimal,
    pub comp_path_fisico: String,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ComprobanteNuevoReq{
    pub numero: String,
    pub valor: f32,
    pub usuario: String,
    pub file: Archivo,
    pub representados: Vec<i32>
}

#[derive(Serialize, Deserialize,Clone)]
pub struct AutorizarComprobante{
    pub numeros: Vec<String>,
    pub usuario: String
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ComprobanteReq{
    pub autorizar: bool,
    pub usuario: String
}