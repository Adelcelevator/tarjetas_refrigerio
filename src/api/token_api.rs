use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::{Serialize, Deserialize};

use crate::{repository::{token_repo::TokenRepo}, models::{response::{Response},token_model::{Token}}};

#[derive(Debug, Serialize, Deserialize)]
struct ValidarToken{
    token:String
}

#[post("/token/validar")]
pub async fn validar_token(db_mongo: Data<TokenRepo>, token:Json<ValidarToken>)->HttpResponse{
    let data = token.token.to_owned();
    let buscado = match db_mongo.buscar_token(&data).await {
        Ok(resultado)=>resultado,
        Err(_)=>Token::init(),
    };
    if buscado.estado != "Activo" {
        let respuesta = Response{
            codigo: Some(403),
            mensaje: Some(String::from("Token no valido")),
            status: None,
        };
        HttpResponse::Forbidden().json(respuesta)
    }else{
        let respuesta = Response{
            codigo: Some(200),
            mensaje: Some(String::from("Ok")),
            status: None,
        };
        HttpResponse::Ok().json(respuesta)
    }
}