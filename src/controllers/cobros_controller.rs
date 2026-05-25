use actix_web::{HttpResponse, post, web::{Data, Json}};
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::{models::data_model::postgres::{pago_model::PagoReq, persona_model::BuscarPersonaCobro}, services::cobros_service::{buscar_persona_cobro, get_tipos_pago_unidades, guardar_pago}, utils::{connection_utils::get_conexion, enums::errors::service_error::ServiceError}};

#[post("/cobros/tiposPago")]
pub async fn tipos_pago_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
        Ok(HttpResponse::Ok()
                      .json(get_tipos_pago_unidades(&mut con).await?))
}

#[post("/cobros/buscarPersona")]
pub async fn buscar_persona_cobro_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    buscar: Json<BuscarPersonaCobro>
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok()
        .json(buscar_persona_cobro(&mut con, buscar.0).await?))
}

#[post("/cobros/cobrar")]
pub async fn cobrar_controller(
    pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
    guardar: Json<PagoReq>
) -> Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok()
            .json(guardar_pago(&mut con,guardar.0).await?))
}