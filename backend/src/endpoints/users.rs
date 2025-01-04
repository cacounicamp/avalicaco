use diesel::result::{DatabaseErrorKind, Error};
use serde::Serialize;

use super::prelude::*;

#[utoipa::path(
    responses(
        (status = 200, description = "List of users", body = [UserRead])
    )
  )]
#[get("/users")]
pub async fn get() -> impl Responder {
    use schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .select(UserRead::as_select())
        .load(connection)
        .expect("Error loading users");
    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    responses(
        (status = 200, body = UserRead),
        (status = 404)
    )
  )]
#[get("/users/{id}")]
pub async fn get_id(path: web::Path<(i32,)>) -> impl Responder {
    use schema::users::dsl::*;
    let (eid, ) = path.into_inner();
    
    let connection = &mut establish_connection();
    let result = users
        .select(UserRead::as_select())
        .filter(id.eq(eid))
        .first(connection);
    match result {
        Ok(eva) => HttpResponse::Ok().json(eva),
        Err(Error::NotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish()
    }
}


#[utoipa::path(
    responses(
        (status = 200),
    )
  )]
#[patch("/users/{id}")]
pub async fn patch(path: web::Path<(i32,)>, body : web::Json<UserUpdateInsert>) -> impl Responder {
    use schema::users::dsl::*;
    let (eid, ) = path.into_inner();
    let req = body.into_inner();
    let connection = &mut establish_connection();
    let result =diesel::update(users.filter(id.eq(eid)))
        .set((
            password_hash.eq(req.password_hash),
            login.eq(req.login)
        ))
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _ )) => HttpResponse::Conflict().body("Login already used"),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

 
#[derive(Serialize, ToSchema)]
struct UserId {
    id: i32
}
#[utoipa::path(
  responses(
      (status = 200, description = "The created user", body = UserId)
  )
)]
#[post("/users")]
pub async fn post(req_body: web::Json<UserUpdateInsert>) -> impl Responder {
    
    use schema::users;
    use schema::users::dsl::*;
    let connection = &mut establish_connection();
    let user = req_body.into_inner();

    let result : Result<i32, Error>  = diesel::insert_into(users::table)
        .values(&user)
        .returning(id)
        .get_result(connection);
    
    match result {
        Ok(uid) => HttpResponse::Ok().json(UserId{id : uid}),
        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _ )) => HttpResponse::Conflict().body("User already  exists"),
        _ => HttpResponse::InternalServerError().finish() 
    }
}


#[utoipa::path(
    responses(
        (status = 200),
    )
  )]
#[delete("/users/{id}")]
pub async fn delete(path: web::Path<(i32,)>) -> impl Responder {
    use schema::users::dsl::*;
    let (eid, ) = path.into_inner();
    let connection = &mut establish_connection();
    let result = diesel::delete(users.filter(id.eq(eid)))
        .execute(connection);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
