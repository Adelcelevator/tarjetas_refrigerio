use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use mongodb::Client;
use crate::{services::token_service::validar_token_service, utils::enums::errors::service_error::ServiceError};

#[get("/token/validar/{token}")]
pub async fn validar_token_controller(db_mongo: Data<Client>, 
                                      path:Path<String>)->Result<HttpResponse,ServiceError>{

    Ok(HttpResponse::Ok().json(validar_token_service(db_mongo.get_ref().clone(),path.as_str()).await?))
}