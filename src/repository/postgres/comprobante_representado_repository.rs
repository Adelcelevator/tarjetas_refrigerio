use crate::{models::data_model::postgres::comprobante_representado_model::ComprobanteRepresentado, utils::enums::errors::service_error::ServiceError};
use log::error;

use super::db_tarjetas_repository::db_tarjetas::tbl_comprobante_representado;

use diesel::insert_into;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn guardar_comprobante_representado(con:&mut AsyncPgConnection,
                                      nuevo:ComprobanteRepresentado) ->Result<usize, ServiceError>{
    let insert = insert_into(tbl_comprobante_representado::dsl::tbl_comprobante_representado)
                                        .values(nuevo)
                                        .execute(con).await;
    
    match insert {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al registrar el comprobante ligado al representado: {}",error);
            Err(ServiceError::BdError("Existio un error al guardar el comprobante relacionado al reprensentante.".to_string()))
        }
    }
}