use diesel::{r2d2::ConnectionManager, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use log::error;

use crate::{models::data_model::tarjeta_historial_model::HistorialTarjeta, repository::db_tarjetas_repository::db_tarjetas::tbl_historial_tarjeta};

pub fn cargar_historial_tarjeta(con:&mut PooledConnection<ConnectionManager<PgConnection>>, numero_tarjeta:&i32)->Vec<HistorialTarjeta>{
    let carga_historial = tbl_historial_tarjeta::dsl::tbl_historial_tarjeta
                                                             .select(tbl_historial_tarjeta::all_columns)
                                                             .filter(tbl_historial_tarjeta::tar_id.eq(*numero_tarjeta))
                                                             .order(tbl_historial_tarjeta::dsl::histo_tar_id.desc())
                                                             .load::<HistorialTarjeta>(con);
    match carga_historial {
        Ok(historial)=>historial,
        Err(error)=>{
            error!("Existio un error al cargar el historial de la tarjeta: {}",error);
            vec![]
        }
    }
}