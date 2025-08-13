use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{models::data_model::tipo_pago_model::DetallePago, repository::db_cobros_repository::db_cobros::tbl_pago};

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_pago)]
pub struct Pago {
    pub  pago_id : Option<i32>,
    pub  per_id : Option<i32>,
    pub  pago_valor_total : f64,
    pub  pago_observacion : Option<String>,
    pub  estado : String,
    pub  usr_creacion : String,
    pub  fe_creacion : NaiveDateTime,
    pub  usr_modificacion : Option<String>,
    pub  fe_modificacion : Option<NaiveDateTime>,
}

impl Pago {
    pub fn void_init()->Pago{
        Pago { pago_id: None, 
               per_id: None, 
               pago_valor_total: 0.0, 
               pago_observacion: None, 
               estado: String::new(), 
               usr_creacion: String::new(), 
               fe_creacion: Local::now().naive_local(), 
               usr_modificacion: None, 
               fe_modificacion: None 
            }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PagoReq {
    pub observacion: Option<String>,
    pub cobrar: f32,
    pub per_id: i32,
    pub unidad: String,
    pub detalle_pagos: Vec<DetallePago>,
    pub usr_registro: String,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct BusquedaPagoReq {
    pub fecha_inicio:String,
    pub fecha_fin:Option<String>,
    pub tipo_pago_id:Option<i32>
}