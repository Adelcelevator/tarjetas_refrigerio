use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, sql_query, sql_types::Integer};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;
use rust_decimal::Decimal;

use crate::{models::data_model::postgres::tarjeta_model::Tarjeta, repository::postgres::db_tarjetas_repository::db_tarjetas::{tbl_representante_representado, tbl_tarjeta, tbl_tarjeta_representado}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

pub async fn cargar_tarjetas_usuario(con:&mut AsyncPgConnection, id_persona:&i32)->Result<Vec<Tarjeta>, ServiceError>{
    
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

    let search = sql_query(str_sql)
            .bind::<Integer,_>(id_persona)
            .load::<Tarjeta>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar las tarjetas del usuario: {}",error);
            Err(ServiceError::BdError("Existio un error al cargar las tarjetas del usuario.".to_string()))
        }
    }
}

pub async fn cargar_saldo_tarjeta(con:&mut AsyncPgConnection, id_persona:&i32)->Result<Vec<Decimal>, ServiceError>{
    let search = tbl_tarjeta::dsl::tbl_tarjeta
                    .inner_join(tbl_tarjeta_representado::dsl::tbl_tarjeta_representado.on(tbl_tarjeta::dsl::tar_id.eq(tbl_tarjeta_representado::dsl::tar_id)))
                    .inner_join(tbl_representante_representado::dsl::tbl_representante_representado.on(tbl_tarjeta_representado::dsl::repre_id.eq(tbl_representante_representado::dsl::repsentado_id)))
                    .filter(tbl_tarjeta::dsl::estado.eq(Estados::Activo.to_string()))
                    .filter(tbl_tarjeta_representado::dsl::estado.eq(Estados::Activo.to_string()))
                    .filter(tbl_representante_representado::dsl::estado.eq(Estados::Activo.to_string()))
                    .filter(tbl_representante_representado::dsl::repsentado_id.eq(id_persona))
                    .select(tbl_tarjeta::tar_saldo)
                    .for_update()
                    .load::<Decimal>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el saldo de la tarjeta: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar el saldo de la tarejeta.".to_string()))
        }
    }
}