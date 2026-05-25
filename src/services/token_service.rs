use mongodb::Client;

use crate::{models::responses::response::Response, repository::mongo::token_repo::buscar_token, utils::{enums::{errors::service_error::ServiceError, estados_enum::Estados}, token_user::validar_token}};

pub async fn validar_token_service(cliente: Client, token: &str) ->Result<Response,ServiceError> {
    if token.is_empty() {
        return Err(ServiceError::MissingToken("Token no encontrado.".to_string()));
    }
    
    let token = urlencoding::encode(token).to_string();
    let Some(res) = buscar_token(cliente,token.as_str()).await? else{
        return Err(ServiceError::MissingToken("No se encontro el token.".to_string()));
    };
    
    if res.estado != Estados::Activo{
        return Err(ServiceError::InvalidToken("Token no valido.".to_string()));
    }

    let claims = validar_token(token.as_str())?;
    if claims.exp == 0{
        return Err(ServiceError::InvalidToken("Token no valido.".to_string()));  
    }

    Ok(Response {
        codigo: Some(200),
        status: Some(String::from("OK")),
        mensaje: Some(String::from("Exitoso")),
    })

}
