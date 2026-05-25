use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_usuario;

#[derive(Queryable,Insertable, Serialize, Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_usuario)]
pub struct UsuarioConsulta {
    pub usu_id: Option<i32>,
    pub usu_usuario: Option<String>,
    pub usu_contra: Option<String>,
    pub estado: Option<String>,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion: Option<String>,
    pub per_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login{
   pub usuario:String,
   pub clave:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePassword{
   pub token:String,
   pub clave:String,
}