use diesel::{ExpressionMethods, query_dsl::methods::FilterDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::usuario_model::UsuarioConsulta, utils::enums::errors::service_error::ServiceError};

use super::db_tarjetas_repository::db_tarjetas::tbl_usuario;
pub async fn buscar_usuario (con:&mut AsyncPgConnection, usuario: String)->Result<UsuarioConsulta, ServiceError>{
    let search = tbl_usuario::dsl::tbl_usuario
        .filter(tbl_usuario::usu_usuario.eq(usuario))
        .first::<UsuarioConsulta>( con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el usuario: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar eel usuario.".to_string()))
        }
    }
}