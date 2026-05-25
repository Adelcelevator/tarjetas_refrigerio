use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_tarjeta;

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, Clone, QueryableByName)]
#[diesel(table_name=tbl_tarjeta)]
pub struct Tarjeta{
    pub tar_id: Option<i32>,
    pub tar_saldo: rust_decimal::Decimal,
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