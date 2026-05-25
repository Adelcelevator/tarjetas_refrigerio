use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::utils::enums::estados_enum::Estados;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token: String,
    pub usuario: String,
    pub estado: Estados,
    #[serde(rename = "fechaRegistro")]
    pub fecha_registro: i64,
}
