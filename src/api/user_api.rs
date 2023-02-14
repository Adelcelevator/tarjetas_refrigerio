use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, put, post,
    web::{Data, Json, Path}, 
    HttpResponse,
};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use mongodb::bson::oid::ObjectId;

#[post("/user/login")]
pub async fn login(db_mongo: Data<TokenRepo>, pool: Data<Pool<ConnectionManager<PgConnection>>>, new_user: Json<User>) -> HttpResponse {
    use crate::models::administrcion_model::administracion::tbl_usuarios::dsl::*;
    let mut conn = pool.get().expect("No se pudo obtener una conexion");
    let result = tbl_usuarios.load::<>(&mut conn);
    let respuesta = match result {
        Ok(resultado)=> resultado,
        Err(_) => vec![],
    };
    HttpResponse::Ok()
    .content_type(constants::APPLICATION_JSON)
    .json(respuesta)
}

/*#[post("/user/logout")]
pub async fn create_user(pool: Data<Pool<ConnectionManager<PgConnection>>>, new_user: Json<User>) -> HttpResponse {
    use crate::models::administrcion_model::administracion::tbl_usuarios::dsl::*;
    let mut conn = pool.get().expect("No se pudo obtener una conexion");
    let result = tbl_usuarios.load::<>(&mut conn);
    let respuesta = match result {
        Ok(resultado)=> resultado,
        Err(_) => vec![],
    };
    HttpResponse::Ok()
    .content_type(constants::APPLICATION_JSON)
    .json(respuesta)
}
*/

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("User with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}