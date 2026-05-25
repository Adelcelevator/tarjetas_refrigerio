use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_roles;

#[derive(Queryable,Insertable, Serialize, Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_roles)]
pub struct RolConsulta {
    pub rol_id:Option<i32>,
    pub rol_rol:Option<String>,
    pub rol_descripcion:Option<String>,
    pub estado:Option<String>,
    pub fe_creacion:NaiveDateTime,
    pub usr_creacion:String,
    pub fe_modificacion:Option<NaiveDateTime>,
    pub usr_modificacion:Option<String>,
}