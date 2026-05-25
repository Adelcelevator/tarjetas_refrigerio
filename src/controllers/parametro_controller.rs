use actix_web::{post, web::{Data, Json}, HttpResponse};
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::{models::data_model::postgres::parametro_det_model::ParametrosReq, services::parametro_service::cargar_parametros, utils::{connection_utils::get_conexion, enums::errors::service_error::ServiceError}};

#[post("/parametros/buscar")]
pub async fn buscar_parametro_controller(pool: Data<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>>,
                        parametros: Json<ParametrosReq>)->Result<HttpResponse,ServiceError> {
    let Some(mut con) = get_conexion(&pool).await else{
        return Ok(HttpResponse::InternalServerError().finish());
    };
    Ok(HttpResponse::Ok().json(cargar_parametros(&mut con, parametros.0).await?))
}