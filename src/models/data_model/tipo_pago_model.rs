use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Queryable,Insertable};
use serde::{Deserialize, Serialize};

use crate::repository::db_cobros_repository::db_cobros::tbl_tipo_pago;

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_tipo_pago)]
pub struct TipoPago {
    pub tipo_pago_id: Option<i32>,
    pub pago_descricion: Option<String>,
    pub estado: String,
    pub usr_creacion: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_modificacion: Option<String>,
    pub fe_modificacion: Option<NaiveDateTime>,
}

impl TipoPago {
    pub fn void_init()-> TipoPago {
        TipoPago{
            tipo_pago_id: None,
            pago_descricion:None,
            estado:String::new(),
            usr_creacion:String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_modificacion: None,
            fe_modificacion:None
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct DetallePago {
    pub tipo_pago_id: i32,
    pub valor_pago: f32,
}