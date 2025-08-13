use diesel::{insert_into, r2d2::ConnectionManager, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;

use crate::{models::data_model::detallle_pago_model::DetallePago, repository::{db_cobros_repository::db_cobros::tbl_detalle_pago}};

use log::error;

pub fn guardar_detalle(conn:&mut PooledConnection<ConnectionManager<PgConnection>>,
                       guardar:Vec<DetallePago>)->bool{
    let guardar = insert_into(tbl_detalle_pago::dsl::tbl_detalle_pago)
                                                .values(&guardar)
                                                .execute(conn);
    match guardar {
        Ok(res)=>{
            if res > 0 {
                return true
            }
            return false
        },
        Err(e)=>{
            error!("Existio un error al guardar el detalle: {}",e);
            return false
        }
    }
}