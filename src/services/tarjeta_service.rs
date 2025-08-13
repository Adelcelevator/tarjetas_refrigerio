use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::{data_model::{tarjeta_historial_model::HistorialTarjeta, tarjeta_model::HistorialTarjetaReq}, responses::response::ResponseData}, repository::tarjeta_historial_repository::cargar_historial_tarjeta, utils::bd_utils::get_conexion};

pub async fn cargar_historial (pool: &Data<Pool<ConnectionManager<PgConnection>>>,
                         tarjeta:HistorialTarjetaReq)-> ResponseData<Vec<HistorialTarjeta>>{
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None
        };
    }
    let mut con = conn.unwrap();

    let historial = cargar_historial_tarjeta(&mut con,&tarjeta.num_tarjeta);

    ResponseData {
        codigo: Some(200),
        status: Some(String::from("Existo")),
        mensaje: Some(String::from("Existo")),
        data: Some(historial)
    }
}