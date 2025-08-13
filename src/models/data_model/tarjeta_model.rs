use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_tarjeta;

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, Clone, QueryableByName)]
#[diesel(table_name=tbl_tarjeta)]
pub struct Tarjeta{
    pub tar_id: Option<i32>,
    pub tar_saldo: f64,
    pub comp_id: Option<i32>,
    pub per_id: Option<i32>,
    pub estado:String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HistorialTarjetaReq {
    pub num_tarjeta: i32,
}

impl Tarjeta {
    pub fn void_init()->Tarjeta{
        Tarjeta{
            tar_id: None,
            tar_saldo:0.0,
            comp_id:None,
            per_id: None,
            estado: String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_creacion: String::new(),
            fe_modificacion: None,
            usr_modificacion: None,
        }
    }
}