use std::env;

use actix_web::{
    cookie::{time::Duration, Cookie, SameSite}, get, post, web::{Data, Json, Path}, HttpResponse
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use crate::{models::data_model::usuario_model::{ChangePassword, Login}, repository::token_repo::TokenRepo, services::usuario_service};

#[post("/user/login")]
pub async fn login_controller(
    db_mongo: Data<TokenRepo>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    login: Json<Login>,
) -> HttpResponse {
    let dominio = env::var("DOMINIO_COOKIE").expect("No se encontro la variable DOMINIO_COOKIE");
    let vida_cookie = env::var("EXPIRACIONTOKENMINUTOS").expect("No se encontro la variable EXPIRACIONTOKENMINUTOS");
    let res = usuario_service::login_service(&db_mongo, &pool, login.0).await;
    if res.codigo.unwrap() != 200 {
        return HttpResponse::Ok()
        .json(res);
    }
    let clone = res.clone();
    let cookie = Cookie::build("token", clone.data.unwrap().token)
                                            .path("/")
                                            .domain(dominio)
                                            .http_only(true)
                                            .same_site(SameSite::Strict)
                                            .max_age(Duration::minutes(vida_cookie.parse::<i64>().unwrap()))
                                            .finish()
                                            ;
    HttpResponse::Ok()
        .cookie(cookie)
        .json(res)
}

#[get("/user/logout/{token}")]
pub async fn logout_controller(
    db_mongo: Data<TokenRepo>,
    path:Path<String>,
) -> HttpResponse {
    HttpResponse::Ok()
        .json(usuario_service::logout_service(&db_mongo, path.as_str()).await)
}

#[post("/user/changePassword")]
pub async fn change_password_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    change_data: Json<ChangePassword>,
) -> HttpResponse {
    HttpResponse::Ok()
        .json(usuario_service::change_password_service(&pool, change_data.0).await)
}