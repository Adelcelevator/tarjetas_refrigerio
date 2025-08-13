use diesel::{
    r2d2::ConnectionManager, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection, QueryDsl, RunQueryDsl
};
use log::error;
use r2d2::PooledConnection;

use crate::{models::data_model::reprensentante_representado_model::RepresentanteRepresentado, repository::db_tarjetas_repository::db_tarjetas::{tbl_persona, tbl_representante_representado, tbl_usuario}};

pub fn cargar_representado_por_usuario(con:&mut PooledConnection<ConnectionManager<PgConnection>>,user:&str)-> Vec<RepresentanteRepresentado>{
    let consulta = tbl_representante_representado::dsl::tbl_representante_representado
                                                      .inner_join(tbl_persona::dsl::tbl_persona.on(tbl_representante_representado::dsl::repsentante_id.eq(tbl_persona::dsl::per_id.nullable())))
                                                      .inner_join(tbl_usuario::dsl::tbl_usuario.on(tbl_persona::dsl::per_id.nullable().eq(tbl_usuario::dsl::per_id)))
                                                      .filter(tbl_usuario::dsl::usu_usuario.eq(user))
                                                      .select(tbl_representante_representado::all_columns)
                                                      .load::<RepresentanteRepresentado>(con);
    match consulta {
        Ok(resultado)=>resultado,
        Err(error) => {
            error!("Existio un error al cargar los representados: {}",error);
            vec![]
        }
    }
}