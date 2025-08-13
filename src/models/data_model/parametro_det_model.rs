use chrono::{Local, NaiveDateTime};
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::repository::db_general_repository::db_general::tbl_parametro_det;

#[derive(Queryable,Insertable,Serialize,Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_parametro_det)]
pub struct ParametroDet{
    pub id_parametro_det: Option<i32> ,
    pub id_parametro_cab: i32,
    pub nombre: String,
    pub parametro_descripcion: Option<String>,
    pub parametro_valor: String,
    pub estado: String,
    pub usr_creacion: String,
    pub fe_creacion: NaiveDateTime,
    pub usr_modificacion:Option<String>,
    pub fe_modificacion: Option<NaiveDateTime>,
}

impl ParametroDet{
    pub fn void_init()-> ParametroDet{
        ParametroDet{
            id_parametro_det: None,
            id_parametro_cab: 0,
            nombre: String::new(),
            parametro_descripcion: None,
            parametro_valor: String::new(),
            estado: String::new(),
            usr_creacion: String::new(),
            fe_creacion: Local::now().naive_local(),
            usr_modificacion:None,
            fe_modificacion:None
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ParametrosReq{
    pub busqueda_parametros: Vec<ParametroReq>
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ParametroReq{
    pub nombre_cabecera: String,
    pub nombre_detalle: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ParametrosRes{
    pub parametros: Vec<ParametroRes>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ParametroRes{
    pub id: i32,
    pub valor: String,
    pub nombre: String,
    pub estado: String
}