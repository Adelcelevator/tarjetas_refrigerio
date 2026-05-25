use diesel::{ExpressionMethods, query_dsl::methods::{FilterDsl, SelectDsl}};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::unidades_model::CargarUnidad, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

use super::db_tarjetas_repository::db_tarjetas::tbl_unidades;

pub async fn get_unidades(con:&mut AsyncPgConnection)->Result<Vec<CargarUnidad>, ServiceError>{
    let search = tbl_unidades::dsl::tbl_unidades.
                        filter(tbl_unidades::dsl::estado.eq(Estados::Activo.to_string()))
                        .select((tbl_unidades::unidad_id,tbl_unidades::unidad_nombre))
                        .load::<CargarUnidad>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar las unidades: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar las unidades.".to_string()))
        }
    }
}