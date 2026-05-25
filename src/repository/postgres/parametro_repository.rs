use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::parametro_det_model::{ParametroDet, ParametroReq}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

use super::db_general_repository::db_general::{tbl_parametro_cab, tbl_parametro_det};

pub async fn cargar_parametro(con:&mut AsyncPgConnection,req: ParametroReq)->Result<Vec<ParametroDet>, ServiceError>{
    let search = tbl_parametro_det::dsl::tbl_parametro_det
                   .inner_join(tbl_parametro_cab::dsl::tbl_parametro_cab.on(tbl_parametro_cab::dsl::id_parametro_cab.assume_not_null()
                                                                            .eq(tbl_parametro_det::dsl::id_parametro_cab)))
                   .filter(tbl_parametro_cab::dsl::nombre_cabecera.eq(req.nombre_cabecera))
                   .filter(tbl_parametro_det::dsl::nombre.eq(req.nombre_detalle))
                   .filter(tbl_parametro_det::dsl::estado.eq(Estados::Activo.to_string()))
                   .select((tbl_parametro_det::id_parametro_det,
                                      tbl_parametro_det::id_parametro_cab,
                                      tbl_parametro_det::nombre,
                                      tbl_parametro_det::parametro_descripcion,
                                      tbl_parametro_det::parametro_valor,
                                      tbl_parametro_det::estado,))
                   .load::<ParametroDet>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el parametro: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar el parametro.".to_string()))
        }
    }
}