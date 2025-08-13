use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use crate::{repository::token_repo::TokenRepo, services::token_service::validar_token_service};

#[get("/token/validar/{token}")]
pub async fn validar_token_controller(db_mongo: Data<TokenRepo>, path:Path<String>)->HttpResponse{
    let validar = validar_token_service(db_mongo.get_ref().clone(),path.as_str());
    HttpResponse::Ok().json(validar.await)
}