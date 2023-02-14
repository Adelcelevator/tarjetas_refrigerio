mod api;
mod models;
mod repository;
use actix_web::{web::Data, App, HttpServer};
use api::{user_api::{ get_user, update_user, delete_user, get_all_users}, token_api::{validar_token}};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use repository::{mongodb_repo::MongoRepo, token_repo::TokenRepo};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("No se encontro la variable DATABASE_URL") ;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool =  Pool::builder().build(manager).expect("No fue posible conectarse a la base de datos");
    let db_mongo = Data::new(MongoRepo::init().await);
    let db_mongo_idp_token = Data::new(TokenRepo::init().await);
    HttpServer::new(move || {
        App::new()
            .app_data(db_mongo.clone())
            .app_data(Data::new(pool.clone()))
            .app_data(db_mongo_idp_token.clone())
            .service(validar_token)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}