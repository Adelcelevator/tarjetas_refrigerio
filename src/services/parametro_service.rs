use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::{data_model::parametro_det_model::{ParametroRes, ParametrosReq, ParametrosRes}, responses::response::ResponseData}, repository::parametro_repository::cargar_parametro, utils::bd_utils::get_conexion};


pub fn cargar_parametros(pool: &Data<Pool<ConnectionManager<PgConnection>>>,
                        parametros:ParametrosReq)-> ResponseData<ParametrosRes>{
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None
        };
    }
    let mut con = conn.unwrap();

    let mut respuesta: Vec<ParametroRes> = vec![];

    for parametro in parametros.busqueda_parametros{
        
        let params= cargar_parametro(&mut con, parametro);
        for para in params{
            respuesta.push(para);
        }
    }
    let res = ParametrosRes{
        parametros:respuesta
    };
    ResponseData { codigo: Some(200), 
                   status: Some(String::from("OK")), 
                   mensaje: Some(String::from("Ok")), 
                   data: Some(res) 
                }
}
