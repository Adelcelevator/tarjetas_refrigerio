mod controllers;
mod models;
mod repository;
mod utils;
mod services;
mod middlewares;
use actix_web::{web::{self,Data}, App, HttpServer};
use controllers::{cobros_controller::{buscar_persona_cobro_controller, tipos_pago_controller}, comprobante_controller::{autorizar_controller, cargar_controller, guardar_comp_controller}, parametro_controller::buscar_parametro_controller, tarjeta_controller::cargar_historial_tarjeta_controller, token_controller::validar_token_controller, user_controller::{change_password_controller, login_controller, logout_controller}};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use middlewares::auth_middleware::MiddleAuthentication;
use repository::token_repo::TokenRepo;
use dotenvy::dotenv;
use std::{env, sync::Arc, time::Duration};

use crate::controllers::cobros_controller::cobrar_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    let database_url = env::var("DATABASE_URL").expect("No se encontro la variable DATABASE_URL") ;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool =  Pool::builder()
                                                                .min_idle(Some(env::var("MIN_POOL_SIZE").expect("No se encontro la variable MIN_POOL_SIZE").parse::<u32>().unwrap()))
                                                                .max_size(env::var("MAX_POOL_SIZE").expect("No se encontro la variable MAX_POOL_SIZE").parse::<u32>().unwrap())
                                                                .connection_timeout(Duration::from_secs(env::var("CONNECTION_TIMEOUT_SECS").expect("No se encontro la variable CONNECTION_TIMEOUT_SECS").parse::<u64>().unwrap()))
                                                                .idle_timeout(Some(Duration::from_secs(env::var("IDLE_TIMEOUT_SECS").expect("No se encontro la variable IDLE_TIMEOUT_SECS").parse::<u64>().unwrap())))
                                                                .build(manager)
                                                                .expect("No fue posible conectarse a la base de datos postgres");
    let db_mongo_idp_token = TokenRepo::init().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(db_mongo_idp_token.clone()))
            .service(web::scope("/tarjetasRefrigerio")
                .service(login_controller)
                .wrap(MiddleAuthentication::new(Arc::new(db_mongo_idp_token.clone())
                                            )
                    ).service(validar_token_controller)
                     .service(logout_controller)
                     .service(change_password_controller)
                     .service(autorizar_controller)
                     .service(cargar_controller)
                     .service(guardar_comp_controller)
                     .service(cargar_historial_tarjeta_controller)
                     .service(tipos_pago_controller)
                     .service(buscar_persona_cobro_controller)
                     .service(buscar_parametro_controller)
                     .service(cobrar_controller)
            )
    })
    .workers(8)
    .bind(("::", 8080))?
    .run()
    .await
}
