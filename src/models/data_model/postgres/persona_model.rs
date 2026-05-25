use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_persona;

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
    pub per_saldo: rust_decimal::Decimal,
    pub unidad_id: Option<i32>,
}
#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct PersonaCobroConsulta{
    pub per_id: Option<i32>,
    pub per_nombre: String,
    pub unidad_nombre: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuscarPersonaCobro{
    pub per_unidad: Option<String>,
    pub per_nombre: Option<String>,
    pub per_identificacion: Option<String>
}