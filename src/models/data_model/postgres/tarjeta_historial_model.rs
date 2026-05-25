use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_historial_tarjeta;

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, Clone)]
#[diesel(table_name=tbl_historial_tarjeta)]
pub struct HistorialTarjeta{
    pub histo_tar_id: Option<i32>,
    pub tar_id: Option<i32>,
    pub repre_id: Option<i32>,
    pub histo_tar_observacion:String,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub tar_saldo: Option<rust_decimal::Decimal>,
}