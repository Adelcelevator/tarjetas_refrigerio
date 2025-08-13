use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_historial_tarjeta;


#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, Clone)]
#[diesel(table_name=tbl_historial_tarjeta)]
pub struct HistorialTarjeta{
    pub histo_tar_id: Option<i32>,
    pub tar_id: Option<i32>,
    pub repre_id: Option<i32>,
    pub histo_tar_observacion:String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub tar_saldo: Option<f64>,
}

impl HistorialTarjeta {
    pub fn void_init()->HistorialTarjeta{
        HistorialTarjeta{
            histo_tar_id:None,
            tar_id: None,
            repre_id: None,
            histo_tar_observacion: String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_creacion: String::new(),
            tar_saldo:None,
        }
    }
}