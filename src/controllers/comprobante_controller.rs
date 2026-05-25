use actix_web::{
    HttpResponse, post, web::{Data, Json}
};

use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::{models::data_model::postgres::comprobante_model::{AutorizarComprobante, ComprobanteNuevoReq, ComprobanteReq}, services::comprobante_service::{autorizar_service, cargar_comp_autorizar_service, cargar_comp_usuario_service, guardar_service}, utils::{connection_utils::get_conexion, enums::errors::service_error::ServiceError}};

#[post("/comprobante/guardar")]
pub async fn guardar_comp_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    comprobante: Json<ComprobanteNuevoReq>,
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok().json(guardar_service(&mut con, comprobante.0).await?))
}

#[post("/comprobante/cargar")]
pub async fn cargar_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    peticion: Json<ComprobanteReq>,
) -> Result<HttpResponse,ServiceError> {
    let autoriza = peticion.0.autorizar;
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };

    if autoriza {
        return Ok(HttpResponse::Ok()
                .json(cargar_comp_autorizar_service(&mut con).await?));
    }

    return Ok(HttpResponse::Ok()
        .json(cargar_comp_usuario_service(&mut con, peticion.0).await?));
}

#[post("/comprobante/autorizar")]
pub async fn autorizar_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    peticion: Json<AutorizarComprobante>,
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok()
        .json(autorizar_service(&mut con, peticion.0).await?))
}