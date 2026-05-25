mod controllers;
mod models;
mod repository;
mod utils;
mod services;
mod middlewares;
mod datasources;
use actix_web::{web::{self,Data}, App, HttpServer};
use controllers::{cobros_controller::{buscar_persona_cobro_controller, tipos_pago_controller}, comprobante_controller::{autorizar_controller, cargar_controller, guardar_comp_controller}, parametro_controller::buscar_parametro_controller, tarjeta_controller::cargar_historial_tarjeta_controller, token_controller::validar_token_controller, user_controller::{change_password_controller, login_controller, logout_controller}, pagos_controller::buscar_pagos_controller};

use middlewares::auth_middleware::MiddleAuthentication;
use dotenvy::dotenv;

use log::error;

use crate::{controllers::cobros_controller::cobrar_controller, datasources::{db_tarjetas::DbTarjetasDataSoruce, db_token::TokenRepo}, middlewares::logging_middleware::LogingStruct, utils::loging_utils};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _guard = loging_utils::init_tracing();
    let pool =  match DbTarjetasDataSoruce::init_pool().await{
        Ok(pol )=>pol,
        Err(error)=>{
            error!("Existio un error al generar el pool: {}",error);
            std::process::exit(1);
        }
    };
    let db_mongo_idp_token = TokenRepo::init().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(db_mongo_idp_token.clone()))
            .wrap(LogingStruct)
            .service(web::scope("/tarjetasRefrigerio")
                .service(login_controller)
                .wrap(MiddleAuthentication::new(db_mongo_idp_token.clone()))
                    .service(validar_token_controller)
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
                    .service(buscar_pagos_controller)
            )
    })
    .workers(8)
    .bind(("::", 8080))?
    .run()
    .await
}
