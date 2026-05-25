use actix_web::{HttpResponse, cookie::{Cookie, SameSite, time::Duration}, post, get, web::{Data, Json}};
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use mongodb::Client;
use log::error;

use crate::{models::data_model::postgres::usuario_model::{ChangePassword, Login}, services::usuario_service::{change_password_service, login_service, logout_service}, utils::{connection_utils::get_conexion, enums::errors::service_error::ServiceError, env_utils::get_variable}};

#[post("/user/login")]
pub async fn login_controller(
    db_mongo: Data<Client>,
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    login: Json<Login>,
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };

    let res = login_service(db_mongo.get_ref().clone(), &mut con, login.0).await?;

    let Some(dominio) = get_variable::<String>("DOMINIO_COOKIE") else {
        error!("No se encontro la variable DOMINIO_COOKIE");
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let Some(vida_cookie) = get_variable::<i64>("EXPIRACIONTOKENMINUTOS") else {
        error!("No se encontro la variable DOMINIO_COOKIE");
        return Ok(HttpResponse::InternalServerError().finish());
    };

    let Some(clone) = res.data.clone() else {
        error!("Existio un error al generar la cookie.");
        return Ok(HttpResponse::InternalServerError().finish());
    };

    let cookie = Cookie::build("token", clone.token)
                                            .path("/")
                                            .domain(dominio)
                                            .http_only(true)
                                            .same_site(SameSite::Strict)
                                            .max_age(Duration::minutes(vida_cookie))
                                            .finish()
                                            ;
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(res))
}

#[get("/user/logout")]
pub async fn logout_controller(
    db_mongo: Data<Client>,
) -> Result<HttpResponse,ServiceError> {
    //TODO CORREGIR EL TOKEN
    Ok(HttpResponse::Ok()
        .json(logout_service(db_mongo.get_ref().clone(), "token").await?))
}

#[post("/user/changePassword")]
pub async fn change_password_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    change_data: Json<ChangePassword>,
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        error!("No se pudo obtener una conexion del pool");
        return Err(ServiceError::InternalServerError);
    };
    Ok(HttpResponse::Ok()
        .json(change_password_service(&mut con, change_data.0).await?))
}