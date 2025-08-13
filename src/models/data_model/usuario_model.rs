use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_usuario;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

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

impl UsuarioConsulta {
    pub fn void_init() -> UsuarioConsulta {
        UsuarioConsulta{
            usu_id:None,
            usu_usuario: Some(String::new()),
            usu_contra :Some(String::new()),
            estado: Some(String::new()),
            fe_creacion: Local::now().naive_local(),
            usr_creacion: String::new(),
            fe_modificacion: Some(Local::now().naive_local()),
            usr_modificacion: Some(String::new()),
            per_id: Some(0),
        }
    }
}