use actix_web::web::Data;
use chrono::Local;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

use crate::{models::{data_model::{detallle_pago_model::DetallePago, pago_model::{Pago, PagoReq}, persona_model::{BuscarPersonaCobro, PersonaCobroConsulta}}, responses::{response::{Response, ResponseData}, tipos_pagos_unidades_response::TiposPagosUnidadesResponse}}, repository::{detalle_pago_repository::guardar_detalle, pago_repository::{anular_pago, guardar_pago_repo}, persona_repository::buscar_persona_para_cobro, tarjeta_repository::cargar_saldo_tarjeta, tipo_pago_repository::get_tipos_pagos, unidades_repository::get_unidades}, utils::{bd_utils::get_conexion, enums::estados_enum::Estados}};


pub async fn get_tipos_pago_unidades(pool:&Data<Pool<ConnectionManager<PgConnection>>>)->ResponseData<TiposPagosUnidadesResponse>{
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None,
        };
    }
    let mut con = conn.unwrap();

    let tipos_pagos = get_tipos_pagos(&mut con);
    let unidades = get_unidades(&mut con);
    if !tipos_pagos.is_empty() &&
       !unidades.is_empty() {
        let res = TiposPagosUnidadesResponse{
            tipos_pagos,
            unidades
        };
        ResponseData {
            codigo: Some(200),
            status: Some(String::from("Exito")),
            mensaje: Some(String::from("Exito")),
            data: Some(res),
        }
    }else{
        ResponseData {
            codigo: Some(500),
            status: Some(String::from("No se pudo obtener los tipos de pagos")),
            mensaje: Some(String::from("No se pudo obtener los tipos de pagos")),
            data: None,
        }
    }
}

pub async fn buscar_persona_cobro(pool:&Data<Pool<ConnectionManager<PgConnection>>>,
                            buscar:BuscarPersonaCobro )->ResponseData<Vec<PersonaCobroConsulta>>{
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None,
        };
    }
    let mut con = conn.unwrap();

    let encontrado = buscar_persona_para_cobro(&mut con, buscar);

    return ResponseData {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
        data: Some(encontrado),
    };
}

pub async fn guardar_pago(pool:&Data<Pool<ConnectionManager<PgConnection>>>,
                          guardar:PagoReq )->Response{
    let mut detalle_guardar:Vec<DetallePago> =vec![]; 
    let conn = get_conexion(pool);
    if conn.is_none(){
        return Response {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
        };
    }
    let mut con = conn.unwrap();

    let valor: f32 = guardar.detalle_pagos.iter().map(|d| d.valor_pago).sum();
    
    if valor > guardar.cobrar {
        return Response {
            codigo: Some(400),
            status: Some(String::from("Error")),
            mensaje: Some(String::from("El valor total del detalle es superior al valor a guardar.")),
        };
    }

    if valor < guardar.cobrar {
        return Response {
            codigo: Some(400),
            status: Some(String::from("Error")),
            mensaje: Some(String::from("El valor total del detalle es inferior al valor a guardar.")),
        };
    }

    let pago = Pago{
        pago_id:None,
        per_id: Some(guardar.per_id.clone()),
        pago_valor_total: guardar.cobrar as f64,
        pago_observacion: guardar.observacion,
        estado: Estados::Activo.to_string(),
        usr_creacion:guardar.usr_registro.clone(),
        fe_creacion: Local::now().naive_local(),
        usr_modificacion: None,
        fe_modificacion: None
    };

    let guardado = guardar_pago_repo(&mut con, pago);

    if guardado.is_none(){
        return Response {
            codigo: Some(500),
            status: Some(String::from("Error")),
            mensaje: Some(String::from("Existio un error al guardar el pago")),
        };
    }

    for detalle in guardar.detalle_pagos {
        if detalle.tipo_pago_id ==2 {
            let saldo = cargar_saldo_tarjeta(&mut con, &guardar.per_id);
            if saldo <= 0.0 {
                anular_pago( &mut con, guardado.clone().unwrap());
                return Response {
                    codigo: Some(400),
                    status: Some(String::from("Error")),
                    mensaje: Some(String::from("La tarjeta no tiene saldo.")),
                };
            }

            if saldo < (detalle.valor_pago as f64) {
                anular_pago( &mut con, guardado.clone().unwrap());
                return Response {
                    codigo: Some(400),
                    status: Some(String::from("Error")),
                    mensaje: Some(String::from(format!("El saldo disponible de la tarjeta es inferior, saldo restante:${}",saldo))),
                };
            }

        }
        detalle_guardar.push(DetallePago { detalle_pago_id: None, 
                                           tipo_pago_id: Some(detalle.tipo_pago_id), 
                                           pago_id: Some(guardado.clone().unwrap()), 
                                           detalle_pago_valor: detalle.valor_pago as f64, 
                                           estado: Estados::Activo.to_string(), 
                                           usr_creacion: guardar.usr_registro.clone(), 
                                           fe_creacion: Local::now().naive_local(), 
                                           usr_modificacion: None, 
                                           fe_modificacion: None 
                                        });
    }

    let guardado_det = guardar_detalle(&mut con, detalle_guardar);

    if !guardado_det {
        anular_pago( &mut con, guardado.clone().unwrap());
        return Response {
            codigo: Some(500),
            status: Some(String::from("Error")),
            mensaje: Some(String::from("Existio un error al guardar el detalle del pago.")),
        };
    }

    return Response {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
    };
}