use chrono::NaiveDateTime;
use diesel::{
    dsl::insert_into, r2d2::ConnectionManager, update, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl
};
use r2d2::PooledConnection;

use crate::{
    models::data_model::pago_model::{BusquedaPagoReq, Pago},
    repository::db_cobros_repository::db_cobros::{tbl_detalle_pago, tbl_pago},
    utils::enums::estados_enum::Estados,
};
use log::error;

pub fn guardar_pago_repo(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    guardar: Pago,
) -> Option<i32> {
    let guardar = insert_into(tbl_pago::dsl::tbl_pago)
        .values(guardar)
        .returning(tbl_pago::pago_id)
        .get_result(conn);
    match guardar {
        Ok(id) => id,
        Err(error) => {
            error!("Existio un error al guardar la informacion: {}", error);
            None
        }
    }
}

pub fn anular_pago(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    id_pago: i32,
) -> bool {
    let actualiza =
        update(tbl_pago::dsl::tbl_pago.filter(tbl_pago::dsl::pago_id.eq(id_pago.clone())))
            .set(tbl_pago::dsl::estado.eq(Estados::Anulado.to_string()))
            .execute(conn);
    match actualiza {
        Ok(res) => {
            if res > 0 {
                return true;
            }
            false
        }
        Err(e) => {
            error!("Existio un error al anular el pago{}:{}", id_pago, e);
            false
        }
    }
}

pub fn buscar_pagos(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    buscar: BusquedaPagoReq,)->Option<Vec<Pago>>{

    let fecha_inicio = NaiveDateTime::parse_from_str(&buscar.fecha_inicio, "%d/%m/%Y %H:%M:%S").expect("Exsitio un error a formatear la fecha de inicio");
    let resultado;
    if buscar.tipo_pago_id.is_some() {
        let mut buscado = tbl_pago::dsl::tbl_pago
                          .inner_join(tbl_detalle_pago::dsl::tbl_detalle_pago.on(tbl_detalle_pago::dsl::pago_id.eq(tbl_pago::dsl::pago_id)))
                          .filter(tbl_detalle_pago::dsl::tipo_pago_id.eq(buscar.tipo_pago_id.unwrap()))
                          .filter(tbl_pago::dsl::fe_creacion.ge(fecha_inicio))
                          .into_boxed();
        if buscar.fecha_fin.is_some() {
            let fecha_fin = NaiveDateTime::parse_from_str(&buscar.fecha_fin.unwrap(), "%d/%m/%Y %H:%M:%S").expect("Exsitio un error a formatear la fecha fin");
            buscado = buscado.filter(tbl_pago::dsl::fe_creacion.le(fecha_fin));
        }
        resultado = buscado.select(tbl_pago::all_columns).load::<Pago>(conn);
    } else {
        let mut buscado = tbl_pago::dsl::tbl_pago
            .filter(tbl_pago::dsl::fe_creacion.ge(fecha_inicio))
            .into_boxed();
        if buscar.fecha_fin.is_some() {
            let fecha_fin = NaiveDateTime::parse_from_str(&buscar.fecha_fin.unwrap(), "%d/%m/%Y %H:%M:%S").expect("Exsitio un error a formatear la fecha fin");
            buscado = buscado.filter(tbl_pago::dsl::fe_creacion.le(fecha_fin));
        }
        resultado = buscado.select(tbl_pago::all_columns).load::<Pago>(conn);
    }

    match resultado {
        Ok(res)=>Some(res),
        Err(error)=>{
            error!("Existio un error al realizar la busqueda de los pagos: {}",error);
            None
        }
    }
}
