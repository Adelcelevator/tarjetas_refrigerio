use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::db_cobros_repository::db_cobros::tbl_detalle_pago;

#[derive(Queryable,Insertable,Serialize,Deserialize
,Debug,Clone)]
#[diesel(table_name=tbl_detalle_pago )]
pub struct DetallePago{
    pub detalle_pago_id : Option<i32>,
    pub tipo_pago_id : Option<i32>,
    pub pago_id : Option<i32>,
    pub detalle_pago_valor : f64,
    pub estado : String,
    pub usr_creacion : String,
    pub fe_creacion : NaiveDateTime,
    pub usr_modificacion : Option<String>,
    pub fe_modificacion : Option<NaiveDateTime>,
}

impl DetallePago{
    pub fn void_init() ->DetallePago{
        DetallePago { 
            detalle_pago_id: None,
            tipo_pago_id: None, 
            pago_id: None, 
            detalle_pago_valor: 0.0, 
            estado: String::new(), 
            usr_creacion: String::new(), 
            fe_creacion: Local::now().naive_local(), 
            usr_modificacion: None, 
            fe_modificacion: None
        }
    }
}