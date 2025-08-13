use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::repository::db_tarjetas_repository::db_tarjetas::tbl_representante_representado;

#[derive(Queryable,Insertable, Serialize, Deserialize,Debug,Clone)]
#[diesel(table_name=tbl_representante_representado)]
pub struct RepresentanteRepresentado {
    pub repre_id: Option<i32>,
    pub repsentante_id: Option<i32>,
    pub repsentado_id: Option<i32>,
    pub estado: Option<String>,
    pub usu_creacion: Option<String>,
    pub fe_creacion: Option<NaiveDateTime>,
    pub usu_modificacion: Option<String>,
    pub fe_modificacion: Option<NaiveDateTime>,
}

impl RepresentanteRepresentado {

    pub fn void_init() -> RepresentanteRepresentado{
        RepresentanteRepresentado{
            repre_id: None,
            repsentado_id: None,
            repsentante_id: None,
            estado:None,
            usu_creacion:None,
            fe_creacion: None,
            usu_modificacion: None,
            fe_modificacion:None
        }
    }

}