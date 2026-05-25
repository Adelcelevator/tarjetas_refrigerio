use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::detallle_pago_model::DetallePago, repository::postgres::db_cobros_repository::db_cobros::tbl_detalle_pago, utils::enums::errors::service_error::ServiceError};

pub async fn guardar_detalle(conn:&mut AsyncPgConnection,
                       guardar:Vec<DetallePago>)->Result<usize, ServiceError>{
    let insert = insert_into(tbl_detalle_pago::dsl::tbl_detalle_pago)
                                                .values(&guardar)
                                                .execute(conn).await;
    match insert {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al registrar el detalle del pago: {}",error);
            Err(ServiceError::BdError("Existio un error al guardar el detalle del pago.".to_string()))
        }
    }
}