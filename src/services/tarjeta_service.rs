use diesel_async::AsyncPgConnection;

use crate::{models::{data_model::postgres::{tarjeta_historial_model::HistorialTarjeta, tarjeta_model::HistorialTarjetaReq}, responses::response::ResponseData}, repository::postgres::tarjeta_historial_repository::cargar_historial_tarjeta, utils::enums::errors::service_error::ServiceError};

pub async fn cargar_historial (con: &mut AsyncPgConnection,
                         tarjeta:HistorialTarjetaReq)->Result<ResponseData<Vec<HistorialTarjeta>>,ServiceError>{
    let historial = cargar_historial_tarjeta(con,&tarjeta.num_tarjeta).await?;

    Ok(ResponseData {
        codigo: Some(200),
        status: Some(String::from("Existo")),
        mensaje: Some(String::from("Existo")),
        data: Some(historial)
    })
}