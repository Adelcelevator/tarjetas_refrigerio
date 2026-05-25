use diesel::{ExpressionMethods, query_dsl::methods::FilterDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::tipo_pago_model::TipoPago, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

use super::db_cobros_repository::db_cobros::tbl_tipo_pago;

pub async fn get_tipos_pagos(con:&mut AsyncPgConnection)->Result<Vec<TipoPago>, ServiceError>{
    let search = tbl_tipo_pago::dsl::tbl_tipo_pago
                .filter(tbl_tipo_pago::dsl::estado.eq(Estados::Activo.to_string()))
                .load::<TipoPago>(con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al cargar los tipos de pagos: {}",error);
            Err(ServiceError::BdError("Existio un error al cargar los tipos de pago.".to_string()))
        }
    }
}