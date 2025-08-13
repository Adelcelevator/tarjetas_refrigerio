use actix_web::{post, web::{Data, Json}, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::data_model::pago_model::BusquedaPagoReq, services::pagos_service};

#[post("/pagos/buscar")]
pub async fn buscar_pagos_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    buscar: Json<BusquedaPagoReq>
) -> HttpResponse {
    HttpResponse::Ok()
        .json(pagos_service::buscar_pago_service(&pool,buscar.0))
}