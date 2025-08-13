use actix_web::{post, web::{Data, Json}, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::data_model::tarjeta_model::HistorialTarjetaReq, services::tarjeta_service};

#[post("/tarjeta/cargarHistorial")]
pub async fn cargar_historial_tarjeta_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    tarjeta: Json<HistorialTarjetaReq>,
) -> HttpResponse {
    HttpResponse::Ok()
        .json(tarjeta_service::cargar_historial(&pool, tarjeta.0).await)
}