use diesel::{
    alias, r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl
};
use r2d2::PooledConnection;

use crate::{repository::db_tarjetas_repository::db_tarjetas::{tbl_roles, tbl_roles_usuario},
            utils::enums::estados_enum::Estados};

use log::error;
pub fn roles_usuario(con:&mut PooledConnection<ConnectionManager<PgConnection>>, id:&i32)->Vec<Option<String>> {

    let (rol, 
        rol_usuario) = alias!(tbl_roles as rol,
                                                  tbl_roles_usuario as rol_usuario);

    let buscado_roles_usuario = rol_usuario
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
        .load::<Option<String>>(con);
    match buscado_roles_usuario {
        Ok(resultado_roles) => resultado_roles,
        Err(error_roles) => {
            error!(
                "Existio un error al buscar los roles del usuario: {}",
                error_roles
            );
            vec![None]
        }
    }
}
