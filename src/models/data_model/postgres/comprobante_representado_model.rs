use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Serialize,Deserialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_comprobante_representado;

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_comprobante_representado)]
pub struct ComprobanteRepresentado{
    pub comprobante_representado_id: Option<i32>,
    pub repre_id: i32,
    pub comp_id: i32,
    pub estado: String,
    pub usr_creacion: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_modificacion: Option<String>,
    pub fe_modificacion: Option<NaiveDateTime>,
}