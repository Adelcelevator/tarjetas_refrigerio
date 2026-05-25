use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_representante_representado;

#[derive(Queryable,Insertable, Serialize, Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_representante_representado)]
pub struct RepresentanteRepresentado {
    pub repre_id: Option<i32>,
    pub repsentante_id: Option<i32>,
    pub repsentado_id: Option<i32>,
    pub estado: Option<String>,
}