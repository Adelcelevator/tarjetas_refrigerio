use diesel::{insert_into, r2d2::ConnectionManager, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;

use crate::models::data_model::comprobante_representado_model::ComprobanteRepresentado;

use super::db_tarjetas_repository::db_tarjetas::tbl_comprobante_representado;

use log::error;

pub fn guardar_comprobante_representado(con:&mut PooledConnection<ConnectionManager<PgConnection>>,
                                      nuevo:ComprobanteRepresentado) ->usize{
    let guardar = insert_into(tbl_comprobante_representado::dsl::tbl_comprobante_representado)
                                        .values(nuevo)
                                        .execute(con);
    match guardar {
        Ok(filas) => filas,
        Err(error) =>{
            error!("Existio un error al guardar la informacion: {}",error);
            0
        }
    }
}