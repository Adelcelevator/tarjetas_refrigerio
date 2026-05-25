use chrono::NaiveDateTime;
use diesel::prelude::{Queryable,Insertable};
use serde::{Deserialize, Serialize};

use crate::repository::postgres::db_cobros_repository::db_cobros::tbl_tipo_pago;

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