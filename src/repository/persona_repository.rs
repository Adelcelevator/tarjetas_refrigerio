use diesel::{
    alias, r2d2::ConnectionManager, ExpressionMethods, JoinOnDsl, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl
};

use log::error;
use r2d2::PooledConnection;

use crate::{models::data_model::persona_model::{BuscarPersonaCobro, PersonaCobroConsulta, PersonaConsulta}, utils::enums::estados_enum::Estados};

use super::db_tarjetas_repository::db_tarjetas::{tbl_persona, tbl_usuario, tbl_unidades};
pub fn buscar_persona(con:&mut PooledConnection<ConnectionManager<PgConnection>>, id: &i32)->PersonaConsulta{
    let buscado_persona = tbl_persona::dsl::tbl_persona
            .filter(tbl_persona::per_id.eq(*id))
            .first::<PersonaConsulta>(con);
            match buscado_persona {
                Ok(resultado_persona) => resultado_persona,
                Err(error) =>{ 
                    error!("Existio un error al buscar la persona: {}",error);
                    PersonaConsulta::void_init()},
            }
}

pub fn buscar_persona_por_usuario(con:&mut PooledConnection<ConnectionManager<PgConnection>>, usuario: &String)->PersonaConsulta{
    let (persona, 
         usu) = alias!(tbl_persona as persona,
                          tbl_usuario as usuario);
    let buscado_persona = persona.inner_join(usu)
                                 .filter(usu.field(tbl_usuario::usu_usuario).eq(usuario))
                                 .select(persona.fields(tbl_persona::all_columns))
                                 .first::<PersonaConsulta>(con)
                                ;
    match buscado_persona {
        Ok(encontrado) => encontrado,
        Err(error)=>{
            error!("Existio un error al buscar a la persona: {}",error);
            PersonaConsulta::void_init()
        }
        
    }
}

pub fn buscar_persona_para_cobro(con:&mut PooledConnection<ConnectionManager<PgConnection>>, 
                                 persona_buscada:BuscarPersonaCobro)->Vec<PersonaCobroConsulta>{
    
    let identi_bus = persona_buscada.per_identificacion.is_some();
    let nombre_bus = persona_buscada.per_nombre.is_some();
    let unidad_bus = persona_buscada.per_unidad.is_some();
    
    if identi_bus && nombre_bus && unidad_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(persona_buscada.per_unidad.unwrap()))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",persona_buscada.per_nombre.unwrap())))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",persona_buscada.per_identificacion.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }

    if nombre_bus && unidad_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(persona_buscada.per_unidad.unwrap()))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",persona_buscada.per_nombre.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }

    if identi_bus && unidad_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(persona_buscada.per_unidad.unwrap()))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",persona_buscada.per_identificacion.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }

    if identi_bus && nombre_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",persona_buscada.per_nombre.unwrap())))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",persona_buscada.per_identificacion.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }
    if nombre_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",persona_buscada.per_nombre.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }
    if identi_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",persona_buscada.per_identificacion.unwrap())))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }
    
    if unidad_bus {
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(persona_buscada.per_unidad.unwrap()))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con);
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return buscado;
    }

    vec![]
}