use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use log::error;

use crate::{models::data_model::comprobante_model::{AutorizarComprobante, ComprobanteNuevoReq, ComprobanteReq}, services::comprobante_service};

#[post("/comprobante/guardar")]
pub async fn guardar_comp_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    comprobante: Json<ComprobanteNuevoReq>,
) -> HttpResponse {
    HttpResponse::Ok()
        .json(comprobante_service::guardar_service(&pool, comprobante.0).await)
}

#[post("/comprobante/cargar")]
pub async fn cargar_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    peticion: Json<ComprobanteReq>,
) -> HttpResponse {
    let autoriza = peticion.0.autorizar;
    if autoriza {
        match comprobante_service::cargar_comp_autorizar_service(&pool).await{
            Some(respuesta) =>HttpResponse::Ok()
            .json(respuesta),
            None=>{
                error!("Existio un error al cargar los comprobantes para autorizar");
                HttpResponse::Forbidden().finish()
            }
        }
    }else{
        match comprobante_service::cargar_comp_usuario_service(&pool, peticion.0).await {
            Some(respuesta)=>{ HttpResponse::Ok()
            .json(respuesta)
        },
            None=>{
                error!("Existio un error al cargar los comprobantes del usuario");
                HttpResponse::Forbidden().finish()
            }
        }
    }
}

#[post("/comprobante/autorizar")]
pub async fn autorizar_controller(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    peticion: Json<AutorizarComprobante>,
) -> HttpResponse {
    HttpResponse::Ok()
        .json(comprobante_service::autorizar_service(&pool, peticion.0).await)
}