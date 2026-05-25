use diesel_async::AsyncPgConnection;

use crate::{models::{data_model::postgres::pago_model::{BusquedaPagoReq, Pago}, responses::response::ResponseData}, repository::postgres::pago_repository::buscar_pagos};

pub async fn buscar_pago_service(con:&mut AsyncPgConnection,
                           buscar:BusquedaPagoReq )->ResponseData<Vec<Pago>>{
    let res = buscar_pagos(con, buscar).await;

    return ResponseData {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
        data:res
    };
}