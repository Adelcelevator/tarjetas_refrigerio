use actix_web::{post, web::{Data, Json}, HttpResponse};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::data_model::{pago_model::PagoReq, persona_model::BuscarPersonaCobro}, services::cobros_service};


#[post("/cobros/tiposPago")]
pub async fn tipos_pago_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>
) -> HttpResponse {
    HttpResponse::Ok()
        .json(cobros_service::get_tipos_pago_unidades(&pool).await)
}

#[post("/cobros/buscarPersona")]
pub async fn buscar_persona_cobro_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    buscar: Json<BuscarPersonaCobro>
) -> HttpResponse {
    HttpResponse::Ok()
        .json(cobros_service::buscar_persona_cobro(&pool, buscar.0).await)
}

#[post("/cobros/cobrar")]
pub async fn cobrar_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    guardar: Json<PagoReq>
) -> HttpResponse {
    HttpResponse::Ok()
        .json(cobros_service::guardar_pago(&pool,guardar.0).await)
}