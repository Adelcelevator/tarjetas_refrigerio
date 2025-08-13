use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_persona;

#[derive(Queryable,Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name=tbl_persona)]
pub struct PersonaConsulta {
    pub per_id: Option<i32>,
    pub per_nombre: String,
    pub per_identificacion:String,
    pub per_telefono: Option<String>,
    pub per_direccion: Option<String>,
    pub estado: Option<String>,
    pub fe_creacion: NaiveDateTime,
    pub usr_creacion: String,
    pub fe_modificacion: Option<NaiveDateTime>,
    pub usr_modificacion: Option<String>,
    pub per_saldo: f64,
    pub unidad_id: Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct PersonaCobroConsulta{
    pub per_id: Option<i32>,
    pub per_nombre: String,
    pub unidad_nombre: String
}

impl PersonaConsulta {
    pub fn void_init() -> PersonaConsulta {
        PersonaConsulta{
            per_id:None,
            per_nombre:String::new(),
            per_identificacion:String::new(),
            per_telefono: Some(String::new()),
            per_direccion: Some(String::new()),
            estado:Some(String::new()),
            fe_creacion:Local::now().naive_local(),
            usr_creacion:String::new(),
            fe_modificacion:Some(Local::now().naive_local()),
            usr_modificacion:Some(String::new()),
            per_saldo:0.0,
            unidad_id: None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuscarPersonaCobro{
    pub per_unidad: Option<String>,
    pub per_nombre: Option<String>,
    pub per_identificacion: Option<String>
}