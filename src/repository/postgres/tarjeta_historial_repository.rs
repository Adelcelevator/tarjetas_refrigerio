use diesel::{ExpressionMethods, query_dsl::methods::{FilterDsl, OrderDsl, SelectDsl}};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::tarjeta_historial_model::HistorialTarjeta, repository::postgres::db_tarjetas_repository::db_tarjetas::tbl_historial_tarjeta, utils::enums::errors::service_error::ServiceError};

pub async fn cargar_historial_tarjeta(con:&mut AsyncPgConnection, numero_tarjeta:&i32)->Result<Vec<HistorialTarjeta>, ServiceError>{
    let search = tbl_historial_tarjeta::dsl::tbl_historial_tarjeta
                                .select(tbl_historial_tarjeta::all_columns)
                                .filter(tbl_historial_tarjeta::tar_id.eq(numero_tarjeta))
                                .order(tbl_historial_tarjeta::dsl::histo_tar_id.desc())
                                .load::<HistorialTarjeta>(con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar al cargar el historial de la tarjeta : {}",error);
            Err(ServiceError::BdError("Existio un error al buscar el historial de la tarjeta.".to_string()))
        }
    }
}