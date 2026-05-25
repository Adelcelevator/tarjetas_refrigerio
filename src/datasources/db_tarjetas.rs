use std::time::Duration;

use bb8::Pool;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use dotenvy::dotenv;
use log::error;

use crate::utils::env_utils::get_variable;

#[derive(Clone)]
pub struct DbTarjetasDataSoruce;

impl DbTarjetasDataSoruce {
    pub async fn init_pool()->Result<Pool<AsyncDieselConnectionManager<AsyncPgConnection>>, diesel_async::pooled_connection::PoolError>{
        dotenv().ok();
        let Some(database_url) = get_variable::<String>("DATABASE_URL") else {
            error!("Existio un error al cargar la variable DATABASE_URL");
            std::process::exit(1);
        };
        let Some(min_pool_size) = get_variable::<u32>("MIN_POOL_SIZE") else{
            error!("Existio un error al cargar la variable MIN_POOL_SIZE");
            std::process::exit(1);
        };
        let Some(max_pool_size) = get_variable::<u32>("MAX_POOL_SIZE") else {
            error!("Existio un error al cargar la variable MAX_POOL_SIZE");
            std::process::exit(1);
        };
        let Some(connection_timeout) = get_variable("CONNECTION_TIMEOUT_SECS") else {
            error!("Existio un error al cargar la variable CONNECTION_TIMEOUT_SECS");
            std::process::exit(1);
        };

        let Some(idle_timeout) = get_variable("IDLE_TIMEOUT_SECS") else {
            error!("Existio un error al cargar la variable IDLE_TIMEOUT_SECS");
            std::process::exit(1);
        };

        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
        Pool::builder()
            .min_idle(Some(min_pool_size))
            .max_size(max_pool_size)
            .connection_timeout(Duration::from_secs(connection_timeout))
            .idle_timeout(Some(Duration::from_secs(idle_timeout)))
            .build(config).await
    }   
}