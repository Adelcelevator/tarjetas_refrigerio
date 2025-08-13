use diesel::{query_dsl::methods::FilterDsl, r2d2::ConnectionManager, ExpressionMethods, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;

use crate::{models::data_model::tipo_pago_model::TipoPago, utils::enums::estados_enum::Estados};

use super::db_cobros_repository::db_cobros::tbl_tipo_pago;
use log::error;

pub fn get_tipos_pagos(con:&mut PooledConnection<ConnectionManager<PgConnection>>)->Vec<TipoPago>{
    let resultado = tbl_tipo_pago::dsl::tbl_tipo_pago
                                                             .filter(tbl_tipo_pago::dsl::estado.eq(Estados::Activo.to_string()))
                                                             .load::<TipoPago>(con);
    match resultado {
        Ok(res) => res,
        Err(error)=>{
            error!("Existio un error al traer los tipos de pago: {}",error);
            vec![]
        }
    }
}