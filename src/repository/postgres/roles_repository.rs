use diesel::{ExpressionMethods, QueryDsl, alias};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{repository::postgres::db_tarjetas_repository::db_tarjetas::{tbl_roles, tbl_roles_usuario}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

pub async fn roles_usuario(con:&mut AsyncPgConnection, id:&i32)->Result<Vec<Option<String>>, ServiceError> {

    let (rol, 
        rol_usuario) = alias!(tbl_roles as rol,
                                                  tbl_roles_usuario as rol_usuario);

    let search = rol_usuario
        .inner_join(rol)
        .filter(
            rol_usuario
                .field(tbl_roles_usuario::usu_id)
                .eq(*id),
        )
        .filter(
            rol_usuario
                .field(tbl_roles_usuario::estado)
                .eq(Estados::Activo.to_string()),
        )
        .select(rol.field(tbl_roles::rol_rol))
        .load::<Option<String>>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar los roles del usuario: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los roles del usuario.".to_string()))
        }
    }
}
