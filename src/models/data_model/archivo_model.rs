use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Archivo{
    pub extension: String,
    pub nombre: String,
    pub contenido: String,
}