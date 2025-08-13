use actix_web::{post, web::{Data, Json}, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::data_model::parametro_det_model::ParametrosReq, services::parametro_service::cargar_parametros};

#[post("/parametros/buscar")]
pub async fn buscar_parametro_controller(pool: Data<Pool<ConnectionManager<PgConnection>>>,
                        parametros: Json<ParametrosReq>)->HttpResponse{
    HttpResponse::Ok().json(cargar_parametros(&pool, parametros.0))
}