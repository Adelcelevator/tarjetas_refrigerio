use chrono::Local;
use diesel::{ExpressionMethods, insert_into, query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl, SelectDsl}, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::comprobante_model::{AutorizarComprobante, ComprobanteConsulta, InsertarComprobante}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

use super::db_tarjetas_repository::db_tarjetas::tbl_comprobantes;

pub async fn nuevo_comprobante(con:&mut AsyncPgConnection, comprobante:InsertarComprobante)->Result<Option<i32>, ServiceError>{

    let insert = insert_into(tbl_comprobantes::dsl::tbl_comprobantes)
                                    .values(&comprobante)
                                    .returning(tbl_comprobantes::dsl::comp_id)
                                    .get_result::<Option<i32>>(con).await;
    match insert {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al guardar el comprobante: {}",error);
            Err(ServiceError::BdError("Existio un error al guardar el comprobante.".to_string()))
        }
    }
}

pub async fn cargar_comprobante_por_usuario(con:&mut AsyncPgConnection,persona_id:&i32)->Result<Vec<ComprobanteConsulta>, ServiceError>{
    let search = tbl_comprobantes::dsl::tbl_comprobantes
                                                            .filter(tbl_comprobantes::dsl::per_id.eq(persona_id))
                                                            .order(tbl_comprobantes::dsl::fe_creacion.desc())
                                                            .limit(10)
                                                            .select((tbl_comprobantes::comp_id,
                                                                                tbl_comprobantes::per_id,
                                                                                tbl_comprobantes::comp_numero,
                                                                                tbl_comprobantes::estado,
                                                                                tbl_comprobantes::fe_creacion,
                                                                                tbl_comprobantes::usr_creacion,
                                                                                tbl_comprobantes::comp_valor,
                                                                                tbl_comprobantes::comp_path_fisico,
                                                                                ))
                                                            .load::<ComprobanteConsulta>(con).await;
    match search {
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar los comprobantes por usuario: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes del usuario.".to_string()))
        }
    }
}

pub async fn cargar_comprobante_por_estado(con:&mut AsyncPgConnection,estado: Estados)->Result<Vec<ComprobanteConsulta>, ServiceError>{
    let search = tbl_comprobantes::dsl::tbl_comprobantes
                                            .filter(tbl_comprobantes::dsl::estado.eq(estado.to_string()))
                                            .order(tbl_comprobantes::dsl::comp_id.desc())
                                            .select((tbl_comprobantes::comp_id,
                                                                                tbl_comprobantes::per_id,
                                                                                tbl_comprobantes::comp_numero,
                                                                                tbl_comprobantes::estado,
                                                                                tbl_comprobantes::fe_creacion,
                                                                                tbl_comprobantes::usr_creacion,
                                                                                tbl_comprobantes::comp_valor,
                                                                                tbl_comprobantes::comp_path_fisico,
                                                                                ))
                                            .load::<ComprobanteConsulta>(con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar los comprobantes por estado: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes.".to_string()))
        }
    }
}

pub async fn autorizar_comprobantes(con:&mut AsyncPgConnection,autorizar:AutorizarComprobante)->(usize,usize){
    let mut numero_posi = 0;
    let mut numero_no = 0;
    let usuario = autorizar.usuario;
    for tp in autorizar.numeros {

        let comprobante = tp.split(",").collect::<Vec<&str>>();

        let numero = match comprobante[1].parse::<i32>(){
            Ok(res)=>res,
            Err(error)=>{
                error!("Existio un error al parsear el id de la persona: {}",error);
                0
            }
        };
        if numero == 0 {
            numero_no+=1;
            continue;
        }
        let respuesta= update(tbl_comprobantes::dsl::tbl_comprobantes
                                                            .filter(tbl_comprobantes::dsl::comp_numero.eq(comprobante[0]))
                                                            .filter(tbl_comprobantes::dsl::per_id.eq(numero))
                                                            .filter(tbl_comprobantes::dsl::estado.eq(Estados::PorAutorizar.to_string()))
                                                    )
                                                    .set((tbl_comprobantes::dsl::estado.eq(Estados::Autorizado.to_string()),
                                                                    tbl_comprobantes::dsl::usr_modificacion.eq(usuario.clone()),
                                                                tbl_comprobantes::dsl::fe_modificacion.eq(Some(Local::now().naive_local()))))
                                                    .execute(con).await;
        match respuesta {
            Ok(cant)=> {
                numero_posi += cant;
            },
            Err(error)=>{
                error!("Error al actualizar los comprobantes: {}",error);
                numero_no+=1;
            }
        }
    }
    (numero_posi,numero_no)
}

pub async fn cargar_comprobante_por_numero_persona(con:&mut AsyncPgConnection,data:(&String,&i32))->Result<Vec<ComprobanteConsulta>, ServiceError>{
    let search = tbl_comprobantes::dsl::tbl_comprobantes
                                            .filter(tbl_comprobantes::dsl::comp_numero.eq(data.0))
                                            .filter(tbl_comprobantes::dsl::per_id.eq(data.1))
                                            .order(tbl_comprobantes::dsl::fe_creacion.desc())
                                            .select((tbl_comprobantes::comp_id,
                                                                                tbl_comprobantes::per_id,
                                                                                tbl_comprobantes::comp_numero,
                                                                                tbl_comprobantes::estado,
                                                                                tbl_comprobantes::fe_creacion,
                                                                                tbl_comprobantes::usr_creacion,
                                                                                tbl_comprobantes::comp_valor,
                                                                                tbl_comprobantes::comp_path_fisico,
                                                                                ))
                                            .load::<ComprobanteConsulta>(con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el comprobante por numero persona: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes.".to_string()))
        }
    }
}