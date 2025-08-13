use diesel::{query_dsl::methods::{FilterDsl, SelectDsl}, r2d2::ConnectionManager, ExpressionMethods, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;
use log::error;

use crate::{models::data_model::unidades_model::CargarUnidad, utils::enums::estados_enum::Estados};

use super::db_tarjetas_repository::db_tarjetas::tbl_unidades;


pub fn get_unidades(con:&mut PooledConnection<ConnectionManager<PgConnection>>)->Vec<CargarUnidad>{
    let cargando = tbl_unidades::dsl::tbl_unidades.
                        filter(tbl_unidades::dsl::estado.eq(Estados::Activo.to_string()))
                        .select((tbl_unidades::unidad_id,tbl_unidades::unidad_nombre))
                        .load::<CargarUnidad>(con);
    match cargando {
        Ok(buscado) => buscado,
        Err(error)=>{
            error!("Existio un error al buscar las unidades: {}",error);
            vec![]
        }
    }
}