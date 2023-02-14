use mongodb::bson::{oid::ObjectId, Timestamp};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token: String,
    pub usuario: String,
    pub estado: String,
    #[serde(rename = "fechaRegistro")]
    pub fecha_registro: Timestamp,
}


impl Token {
    pub fn init()->Token{
        Token { id: None,
             token: String::from(""),
             usuario: String::from(""), 
             estado: String::from(""), 
             fecha_registro: Timestamp { time: 0, increment: 0 },
             }
    }
}