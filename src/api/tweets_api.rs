use actix_web::{get, post, HttpResponse, web::{Path, Data}};
use diesel::{ Insertable, Queryable, RunQueryDsl,r2d2::{Pool, ConnectionManager}, PgConnection};
use chrono::{NaiveDateTime, Utc};
use crate::models::schema::tweets;
use serde::{Serialize, Deserialize};
// localhost:8080/tweets-> GET: obtiene tweets POST: se crea un tweet
// localhost:8080/tweets/:id-> GET: obtiene tweets con id   DELETE: se borra el tweet
// localhost:8080/tweets/:id/likes ->   GET:Obtiene los likes de ese tweet
//                                      POST: Da like al tweet
//                                      DELETE: elimina el like
//################### /tweets ###################################
#[derive(Queryable,Insertable, Serialize, Deserialize)]
#[diesel(table_name=tweets)]
struct TweetI {
    id:Option<i32>,
    create_at:NaiveDateTime,
    mensaje:String
}

#[derive(Queryable,Insertable, Serialize, Deserialize)]
#[diesel(table_name=tweets)]
struct TweetC {
    id:i32,
    create_at:NaiveDateTime,
    mensaje:String
}

impl TweetI {
    
    fn new(mensaje:String)->Self{
        Self { 
            id: Option::None , 
            create_at: Utc::now().naive_utc(), 
            mensaje: mensaje
         }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>)->HttpResponse{
    use crate::models::schema::tweets::dsl::*;
    let mut conn = pool.get().expect("No se pudo obtener una conexion");
    let result = tweets.load::<TweetC>(&mut conn);
    let respuesta = match result {
        Ok(resultado)=> resultado,
        Err(_) => vec![],
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .json(respuesta)
}

#[post("/tweets")]
pub async fn create_tweet(req_body:String, pool: Data<Pool<ConnectionManager<PgConnection>>>)->HttpResponse{
    let nuevo_tweet = TweetI::new(req_body);
    let mut conn = pool.get().expect("No se pudo obtener una conexion");
    diesel::insert_into(tweets::table)
    .values(&nuevo_tweet)
    .execute(&mut conn).expect("Error al insertar el tweet");
    HttpResponse::Created()
    .content_type("application/json")
    .json(&nuevo_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>)->HttpResponse{
    // TODO get tweet por id
    let tweet = format!("ESTE NO ES EL TWEET {:?}",path.0);
    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweet)
}