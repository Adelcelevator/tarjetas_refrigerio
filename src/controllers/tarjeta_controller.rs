use actix_web::{post, web::{Data, Json}, HttpResponse};
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::{models::data_model::postgres::tarjeta_model::HistorialTarjetaReq, services::tarjeta_service::cargar_historial, utils::{connection_utils::get_conexion, enums::errors::service_error::ServiceError}};

#[post("/tarjeta/cargarHistorial")]
pub async fn cargar_historial_tarjeta_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    tarjeta: Json<HistorialTarjetaReq>,
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok()
        .json(cargar_historial(&mut con, tarjeta.0).await?))
}