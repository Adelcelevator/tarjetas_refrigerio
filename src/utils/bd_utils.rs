use std::time::Duration;

use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};
use log::{error, info};
use r2d2::{Pool, PooledConnection};

pub fn get_conexion(pool: &Data<Pool<ConnectionManager<PgConnection>>>)->
Option<PooledConnection<ConnectionManager<PgConnection>>>{
    
    for i in 1 .. 6 {
        let con = conectar(pool);
        if con.is_some() {
            return con;
        }else if i > 1 {
            info!("Numero de reintento para conetar a base {}", i);
        }
        std::thread::sleep(Duration::from_millis(10*i));
    }
    return None;
}

fn conectar(pool: &Data<Pool<ConnectionManager<PgConnection>>>)->
 Option<PooledConnection<ConnectionManager<PgConnection>>> {
    let intento = pool.get();
    match intento {
        Ok(co)=> Some(co),
        Err(error)=>{
            error!("Existio un error al traer la conexion: {}",error);
            None
        }
    }
}