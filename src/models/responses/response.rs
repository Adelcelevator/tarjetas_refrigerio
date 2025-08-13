use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response{
    pub codigo: Option<i32>,
    pub status: Option<String>,
    pub mensaje: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseData<T>{
    pub codigo: Option<i32>,
    pub status: Option<String>,
    pub mensaje: Option<String>,
    pub data: Option<T>,
}