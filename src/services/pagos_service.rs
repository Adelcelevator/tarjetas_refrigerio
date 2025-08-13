use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::{data_model::pago_model::{BusquedaPagoReq, Pago}, responses::response::ResponseData}, repository::pago_repository::buscar_pagos, utils::bd_utils::get_conexion};


pub fn buscar_pago_service(pool:&Data<Pool<ConnectionManager<PgConnection>>>,
                           buscar:BusquedaPagoReq )->ResponseData<Vec<Pago>>{
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data:None
        };
    }
    let mut con = conn.unwrap();
    let res = buscar_pagos(&mut con, buscar);

    return ResponseData {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
        data:res
    };
}