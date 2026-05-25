use bb8::{Pool, PooledConnection};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use log::error;

pub async fn get_conexion(
    pool: &Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
) -> Option<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>> {
    let conn = pool.get().await;

    match conn {
        Ok(res) => Some(res),
        Err(error) => {
            error!("Existio un error al obtener una conexion del pool: {}", error);
            None
        }
    }
}