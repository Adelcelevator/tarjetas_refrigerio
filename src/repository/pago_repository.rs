use chrono::Local;
use diesel::{dsl::insert_into, query_dsl::methods::FilterDsl, r2d2::ConnectionManager, update, ExpressionMethods, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;

use crate::{models::data_model::pago_model::{BusquedaPagoReq, Pago}, repository::db_cobros_repository::db_cobros::tbl_pago, utils::enums::estados_enum::Estados};
use log::error;

pub fn guardar_pago_repo(conn:&mut PooledConnection<ConnectionManager<PgConnection>>,
                    guardar:Pago) ->Option<i32>{
    let guardar = insert_into(tbl_pago::dsl::tbl_pago)
                                        .values(guardar)
                                        .returning(tbl_pago::pago_id)
                                        .get_result(conn);
    match guardar {
        Ok(id) => id,
        Err(error) =>{
            error!("Existio un error al guardar la informacion: {}",error);
            None
        }
    }
}

pub fn anular_pago(conn:&mut PooledConnection<ConnectionManager<PgConnection>>,
                   id_pago:i32) ->bool{
    let actualiza = update(tbl_pago::dsl::tbl_pago.filter(tbl_pago::dsl::pago_id.eq(id_pago.clone())))
                                                .set(tbl_pago::dsl::estado.eq(Estados::Anulado.to_string()))
                                                .execute(conn);
    match actualiza {
        Ok(res)=>{
            if res > 0{
                return true;
            }
            false
        },
        Err( e)=>{
            error!("Existio un error al anular el pago{}:{}",id_pago,e);
            false
        }
    }

}

pub fn buscar_pagos(conn:&mut PooledConnection<ConnectionManager<PgConnection>>,
                    buscar:BusquedaPagoReq ){
    
}