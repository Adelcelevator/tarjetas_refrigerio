use chrono::Local;
use diesel::{insert_into, query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl, SelectDsl}, r2d2::ConnectionManager, update, ExpressionMethods, PgConnection, RunQueryDsl};
use log::{debug, error};
use r2d2::PooledConnection;

use crate::{models::data_model::comprobante_model::{ComprobanteConsulta, ComprobanteNuevo}, utils::enums::estados_enum::Estados};

use super::db_tarjetas_repository::db_tarjetas::tbl_comprobantes;

pub fn nuevo_comprobante(con:&mut PooledConnection<ConnectionManager<PgConnection>>, comprobante:ComprobanteNuevo)-> i32
{

    let com_guardar:ComprobanteConsulta = ComprobanteConsulta{
        comp_id:None,
        comp_numero : comprobante.numero,
        comp_valor : comprobante.valor,
        comp_path_fisico: comprobante.file,
        usr_creacion: comprobante.usuario,
        fe_creacion: Local::now().naive_local(),
        per_id: Some(comprobante.persona_id),
        estado: Estados::PorAutorizar.to_string(),
        fe_modificacion: None,
        usr_modificacion: None
    };

    let res:Result<Option<i32>, diesel::result::Error>= insert_into(tbl_comprobantes::dsl::tbl_comprobantes)
                                    .values(&com_guardar)
                                    .returning(tbl_comprobantes::dsl::comp_id)
                                    .get_result(con);
    match res {
        Ok(id)=> {
            if id.is_some(){
                id.unwrap()
            }else{
                error!("Existio un error al obtener el id del comprobante");
                0 
            }
        },
        Err(error)=>{
            error!("Existio un error al guardar el comprobante: {}",error);
            0
        }
    }
}

pub fn cargar_comprobante_por_usuario(con:&mut PooledConnection<ConnectionManager<PgConnection>>,persona_id:&i32)->Vec<ComprobanteConsulta>{
    let respuesta: Result<Vec<ComprobanteConsulta>, _> = tbl_comprobantes::dsl::tbl_comprobantes.select(tbl_comprobantes::all_columns)
                                                            .filter(tbl_comprobantes::dsl::per_id.eq(persona_id))
                                                            .order(tbl_comprobantes::dsl::fe_creacion.desc())
                                                            .limit(10)
                                                            .load::<ComprobanteConsulta>(con);
    match respuesta {
        Ok( mut encontrado) =>{
            for c in encontrado.iter_mut() {
                c.comp_id = None;
            }
            encontrado
        },
        Err(error) => {
            error!("Existio un error al cargar los comprobantes por usuario: {}",error);
            Vec::new()
        }
    }
}

pub fn cargar_comprobante_por_estado(con:&mut PooledConnection<ConnectionManager<PgConnection>>,estado: Estados)->Vec<ComprobanteConsulta>{
    let respuesta: Result<Vec<ComprobanteConsulta>, _> = tbl_comprobantes::dsl::tbl_comprobantes.select(tbl_comprobantes::all_columns)
                                                            .filter(tbl_comprobantes::dsl::estado.eq(estado.to_string()))
                                                            .order(tbl_comprobantes::dsl::comp_id.desc())
                                                            .load::<ComprobanteConsulta>(con);
    match respuesta {
        Ok( mut encontrado) =>{
            for c in encontrado.iter_mut() {
                c.comp_id = None;
            }
            encontrado
        },
        Err(error) => {
            error!("Exisito un erro al cargar los comprobantes por estado: {}",error);
            Vec::new()
        }
    }
}

pub fn autorizar_comprobantes(con:&mut PooledConnection<ConnectionManager<PgConnection>>,numeros_comprobantes:Vec<(String,String)>)->(usize,usize){
    let mut numero_posi:usize = 0;
    let mut numero_no:usize = 0;
    for tp in numeros_comprobantes {
        let comprobante = tp.0.split(",").collect::<Vec<&str>>();
        let respuesta: Result<usize, _> = update(tbl_comprobantes::dsl::tbl_comprobantes.filter(tbl_comprobantes::dsl::comp_numero.eq(comprobante[0])))
                                      .set((tbl_comprobantes::dsl::estado.eq(Estados::Autorizado.to_string()),
                                                    tbl_comprobantes::dsl::usr_modificacion.eq(tp.1),
                                                    tbl_comprobantes::dsl::per_id.eq(comprobante[1].parse::<i32>().unwrap()),
                                                tbl_comprobantes::dsl::fe_modificacion.eq(Some(Local::now().naive_local()))))
                                      .execute(con);
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

pub fn cargar_comprobante_por_numero_persona(con:&mut PooledConnection<ConnectionManager<PgConnection>>,data:(String,i32))->ComprobanteConsulta{
    let respuesta = tbl_comprobantes::dsl::tbl_comprobantes.select(tbl_comprobantes::all_columns)
                                                            .filter(tbl_comprobantes::dsl::comp_numero.eq(data.0))
                                                            .filter(tbl_comprobantes::dsl::per_id.eq(data.1))
                                                            .order(tbl_comprobantes::dsl::fe_creacion.desc())
                                                            .first::<ComprobanteConsulta>(con);
    match respuesta {
        Ok( mut encontrado) =>{
            encontrado.comp_id = None;
            encontrado
        },
        Err(error) => {
            debug!("Error al buscar comprobnate por numero y persona: {}",error);
            ComprobanteConsulta::void_init()
        }
    }
}