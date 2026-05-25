use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};


#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct CargarUnidad{
    unidad_id:Option<i32>,
    unidad_nombre:String
}