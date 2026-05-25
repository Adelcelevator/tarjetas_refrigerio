use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{models::data_model::postgres::detallle_pago_model::DetallePagoReq, repository::postgres::db_cobros_repository::db_cobros::tbl_pago};

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_pago)]
pub struct Pago {
    pub  pago_id : Option<i32>,
    pub  per_id : Option<i32>,
    pub  pago_valor_total : rust_decimal::Decimal,
    pub  pago_observacion : Option<String>,
    pub  estado : String,
    pub  usr_creacion : String,
    pub  fe_creacion : NaiveDateTime,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PagoReq {
    pub observacion: Option<String>,
    pub cobrar: f32,
    pub per_id: i32,
    pub unidad: String,
    pub detalle_pagos: Vec<DetallePagoReq>,
    pub usr_registro: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct BusquedaPagoReq {
    pub fecha_inicio:String,
    pub fecha_fin:Option<String>,
    pub tipo_pago_id:Option<i32>
}