use diesel::{r2d2::ConnectionManager, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection, QueryDsl,RunQueryDsl};
use r2d2::PooledConnection;

use log::error;

use crate::{models::data_model::parametro_det_model::{ParametroDet, ParametroReq, ParametroRes}, utils::enums::estados_enum::Estados};

use super::db_general_repository::db_general::{tbl_parametro_cab, tbl_parametro_det};

pub fn cargar_parametro(con:&mut PooledConnection<ConnectionManager<PgConnection>>,req: ParametroReq)->Vec<ParametroRes>{
    let buscando = tbl_parametro_det::dsl::tbl_parametro_det
                   .inner_join(tbl_parametro_cab::dsl::tbl_parametro_cab.on(tbl_parametro_cab::dsl::id_parametro_cab.assume_not_null()
                                                                            .eq(tbl_parametro_det::dsl::id_parametro_cab)))
                   .filter(tbl_parametro_cab::dsl::nombre_cabecera.eq(req.nombre_cabecera))
                   .filter(tbl_parametro_det::dsl::nombre.eq(req.nombre_detalle))
                   .filter(tbl_parametro_det::dsl::estado.eq(Estados::Activo.to_string()))
                   .select(tbl_parametro_det::all_columns)
                   .load::<ParametroDet>(con);
    
    match buscando {
        Ok(arr)=>{
            let mut arrp = vec![];
            for par in arr{
                arrp.push(ParametroRes{
                    id: par.id_parametro_det.unwrap(),
                    estado: par.estado,
                    valor: par.parametro_valor,
                    nombre:par.nombre
                });
            }
            arrp
        },
        Err(error)=>{
            error!("Existio un error al consultar el parametro: {}",error);
            vec![]
        }
    }
}