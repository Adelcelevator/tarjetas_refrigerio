use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_roles;

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

impl RolConsulta {
    pub fn void_init() -> RolConsulta {
        RolConsulta{
            rol_id: None,
            rol_rol: Some(String::new()),
            rol_descripcion:Some(String::new()),
            estado: Some(String::new()),
            usr_creacion: String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_modificacion: Some(String::new()),
            fe_modificacion: Some(Local::now().naive_local()),            
        }
    }
}