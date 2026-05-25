use actix_web::{post, web::{Data, Json}, HttpResponse};
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::{models::data_model::postgres::pago_model::BusquedaPagoReq, services::pagos_service::buscar_pago_service, utils::connection_utils::get_conexion};

#[post("/pagos/buscar")]
pub async fn buscar_pagos_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    buscar: Json<BusquedaPagoReq>
) -> HttpResponse {
    let Some(mut con) = get_conexion(&pool).await else{
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok()
        .json(buscar_pago_service(&mut con,buscar.0).await)
}