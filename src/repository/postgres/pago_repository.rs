use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, dsl::update, insert_into};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::pago_model::{BusquedaPagoReq, Pago}, repository::postgres::db_cobros_repository::db_cobros::{tbl_detalle_pago, tbl_pago}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

pub async fn guardar_pago_repo(
    conn: &mut AsyncPgConnection,
    guardar: Pago,
) ->Result<Option<i32>, ServiceError> {
    let insert = insert_into(tbl_pago::dsl::tbl_pago)
                                            .values(guardar)
                                            .returning(tbl_pago::pago_id)
                                            .get_result(conn).await;
    
    match insert {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al guardar el pago: {}",error);
            Err(ServiceError::BdError("Existio un error al guardar del pago.".to_string()))
        }
    }
}

pub async fn anular_pago(
    conn: &mut AsyncPgConnection,
    id_pago: i32,
) -> Result<usize, ServiceError> {
    let update = update(tbl_pago::dsl::tbl_pago.filter(tbl_pago::dsl::pago_id.eq(id_pago.clone())))
            .set(tbl_pago::dsl::estado.eq(Estados::Anulado.to_string()))
            .execute(conn).await;

    match update {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al anular el pago {}: {}", id_pago,error);
            Err(ServiceError::BdError("Existio un error al guardar del pago.".to_string()))
        }
    }
}

pub async fn buscar_pagos(
    conn: &mut AsyncPgConnection,
    buscar: BusquedaPagoReq,)->Option<Vec<Pago>>{

    let Ok(fecha_inicio) = NaiveDateTime::parse_from_str(&buscar.fecha_inicio, "%d/%m/%Y %H:%M:%S") else {
        error!("Existio un error a formatear la fecha de inicio");
        return None;
    };

    let fecha_fin = if let Some(f) = &buscar.fecha_fin {
        let Ok(parsed) = NaiveDateTime::parse_from_str(f, "%d/%m/%Y %H:%M:%S") else {
            error!("Existio un error a formatear la fecha fin");
            return None;
        };
        Some(parsed)
    } else {
        None
    };

    let resultado = if let Some(tipo_id) = buscar.tipo_pago_id {
        let mut buscado = tbl_pago::dsl::tbl_pago
            .inner_join(tbl_detalle_pago::dsl::tbl_detalle_pago.on(tbl_detalle_pago::dsl::pago_id.eq(tbl_pago::dsl::pago_id)))
            .filter(tbl_detalle_pago::dsl::tipo_pago_id.eq(tipo_id))
            .filter(tbl_pago::dsl::fe_creacion.ge(fecha_inicio))
            .into_boxed();

        if let Some(ff) = fecha_fin {
            buscado = buscado.filter(tbl_pago::dsl::fe_creacion.le(ff));
        }
        buscado.select((tbl_pago::pago_id,
                                  tbl_pago::per_id,
                                  tbl_pago::pago_valor_total,
                                  tbl_pago::pago_observacion,
                                  tbl_pago::estado,
                                  tbl_pago::usr_creacion,
                                  tbl_pago::fe_creacion)).load::<Pago>(conn).await
    } else {
        let mut buscado = tbl_pago::dsl::tbl_pago
            .filter(tbl_pago::dsl::fe_creacion.ge(fecha_inicio))
            .into_boxed();

        if let Some(ff) = fecha_fin {
            buscado = buscado.filter(tbl_pago::dsl::fe_creacion.le(ff));
        }
        buscado.select((tbl_pago::pago_id,
                                  tbl_pago::per_id,
                                  tbl_pago::pago_valor_total,
                                  tbl_pago::pago_observacion,
                                  tbl_pago::estado,
                                  tbl_pago::usr_creacion,
                                  tbl_pago::fe_creacion)).load::<Pago>(conn).await
    };

    let res = match resultado {
        Ok(res) => res,
        Err(error) => {
            error!("Existio un error al realizar la busqueda de los pagos: {}", error);
            return None;
        }
    };
    Some(res)
}
