use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl
};

use log::error;
use r2d2::PooledConnection;

use crate::models::data_model::usuario_model::UsuarioConsulta;

use super::db_tarjetas_repository::db_tarjetas::tbl_usuario;
pub fn buscar_usuario (con:&mut PooledConnection<ConnectionManager<PgConnection>>, usuario: String)->UsuarioConsulta{
    let buscado = tbl_usuario::dsl::tbl_usuario
        .filter(tbl_usuario::usu_usuario.eq(usuario))
        .first::<UsuarioConsulta>( con);
    match buscado {
        Ok(resultado) => resultado,
        Err(error) => {
            error!("Existio un error al buscar el usuario: {}",error);
            UsuarioConsulta::void_init()
        },
    }
}