use diesel::{ r2d2::ConnectionManager, sql_query, sql_types::Integer, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::PooledConnection;

use log::error;

use crate::{models::data_model::tarjeta_model::Tarjeta, repository::db_tarjetas_repository::db_tarjetas::{tbl_representante_representado, tbl_tarjeta, tbl_tarjeta_representado}, utils::enums::estados_enum::Estados};

pub fn cargar_tarjetas_usuario(con:&mut PooledConnection<ConnectionManager<PgConnection>>, id_persona:&i32)->Vec<Tarjeta>{
    
    let str_sql = r#"SELECT * FROM ( (SELECT tar.* FROM db_tarjetas.tbl_tarjeta tar
                        INNER JOIN db_tarjetas.tbl_tarjeta_representado tre ON tre.tar_id = tar.tar_id
                        INNER JOIN db_tarjetas.tbl_representante_representado repre ON repre.repre_id = tre.repre_id
                    WHERE
                        repre.repsentante_id = $1) UNION
                        (SELECT tar.* FROM db_tarjetas.tbl_tarjeta tar
                        INNER JOIN db_tarjetas.tbl_tarjeta_representado tre ON tre.tar_id = tar.tar_id
                        INNER JOIN db_tarjetas.tbl_representante_representado repre ON repre.repre_id = tre.repre_id
                    WHERE
                        repre.repsentado_id = $1)) ORDER BY tar_id DESC;
                    "#;

    let carga_tarjetas = sql_query(str_sql)
                                                          .bind::<Integer,_>(id_persona)
                                                          .load::<Tarjeta>(con)
                                                          ;
    
    match carga_tarjetas {
        Ok(tarjetas) => tarjetas ,
        Err(error)=>{
            error!("Existio un error al buscar las tarjetas: {}",error);
            vec![]
        }
    }
}

pub fn cargar_saldo_tarjeta(con:&mut PooledConnection<ConnectionManager<PgConnection>>, id_persona:&i32)->f64{
    let buscado = tbl_tarjeta::dsl::tbl_tarjeta
                                            .inner_join(tbl_tarjeta_representado::dsl::tbl_tarjeta_representado.on(tbl_tarjeta::dsl::tar_id.eq(tbl_tarjeta_representado::dsl::tar_id)))
                                            .inner_join(tbl_representante_representado::dsl::tbl_representante_representado.on(tbl_tarjeta_representado::dsl::repre_id.eq(tbl_representante_representado::dsl::repsentado_id)))
                                            .filter(tbl_tarjeta::dsl::estado.eq(Estados::Activo.to_string()))
                                            .filter(tbl_tarjeta_representado::dsl::estado.eq(Estados::Activo.to_string()))
                                            .filter(tbl_representante_representado::dsl::estado.eq(Estados::Activo.to_string()))
                                            .filter(tbl_representante_representado::dsl::repsentado_id.eq(id_persona))
                                            .select(tbl_tarjeta::tar_saldo)
                                            .load::<f64>(con);
    match buscado {
        Ok(res)=>{
            if res.len() > 0 {
                return res[0];
            }else{
                return 0.0;
            }
        },
        Err(error)=>{
            error!("Existio un error al buscar el saldo: {}",error);
            0.0
        }
    }
}