use crate::{models::responses::response::Response, repository::token_repo::TokenRepo, utils::{enums::estados_enum::Estados, token_user::validar_token}};

pub async fn validar_token_service(db_mongo: TokenRepo, token: &str) -> Response {
    if token.is_empty() {
        return Response {
            codigo: Some(401),
            status: Some(String::from("Sin autorizacion")),
            mensaje: Some(String::from("Token no valido")),
        };
    }
    let token = urlencoding::encode(token).to_string();
    let res = db_mongo
        .buscar_token(token.as_str())
        .await;
    if res.is_none() {
        Response {
            codigo: Some(404),
            status: Some(String::from("No encontrado")),
            mensaje: Some(String::from("No se encontro el token")),
        }  
    }else{
        if res.unwrap().estado != Estados::Activo{
            return Response {
                codigo: Some(401),
                status: Some(String::from("Sin autorizacion")),
                mensaje: Some(String::from("Token no valido")),
            };    
        }
        let claims = validar_token(token.as_str());
        if claims.exp ==0{
            return Response {
                codigo: Some(401),
                status: Some(String::from("Sin autorizacion")),
                mensaje: Some(String::from("Token no valido")),
            };    
        }
        Response {
            codigo: Some(200),
            status: Some(String::from("OK")),
            mensaje: Some(String::from("Exitoso")),
        }
    }
}
