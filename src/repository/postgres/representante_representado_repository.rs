use diesel::{ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::reprensentante_representado_model::RepresentanteRepresentado, repository::postgres::db_tarjetas_repository::db_tarjetas::{tbl_persona, tbl_representante_representado, tbl_usuario}, utils::enums::errors::service_error::ServiceError};

pub async fn cargar_representado_por_usuario(con:&mut AsyncPgConnection,user:&str)->Result<Vec<RepresentanteRepresentado>, ServiceError>{
    let search =tbl_representante_representado::dsl::tbl_representante_representado
                                        .inner_join(tbl_persona::dsl::tbl_persona.on(tbl_representante_representado::dsl::repsentante_id.eq(tbl_persona::dsl::per_id.nullable())))
                                        .inner_join(tbl_usuario::dsl::tbl_usuario.on(tbl_persona::dsl::per_id.nullable().eq(tbl_usuario::dsl::per_id)))
                                        .filter(tbl_usuario::dsl::usu_usuario.eq(user))
                                        .select((tbl_representante_representado::repre_id,
                                                            tbl_representante_representado::repsentante_id,
                                                            tbl_representante_representado::repsentado_id,
                                                            tbl_representante_representado::estado))
                                        .load::<RepresentanteRepresentado>(con).await;
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el comprobante por numero persona: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes.".to_string()))
        }
    }
}