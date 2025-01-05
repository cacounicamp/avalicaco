use super::prelude::*;
use crate::auth::{token_encode, UserIdentity};

#[derive(Deserialize, ToSchema)]
struct LoginRequest {
    pub login: String,
    pub password_hash: String,
}

#[derive(Serialize, ToSchema)]
struct LoginResponse {
    jwt_token: String,
}

#[utoipa::path(
    responses((status = 200))
)]
#[post("/auth/login")]
async fn login(body: web::Json<LoginRequest>) -> impl Responder {
    use schema::users::dsl::*;
    let log = body.login.trim().to_lowercase();
    let pass = body.into_inner().password_hash;
    let connection = &mut establish_connection();
    let result: Result<i32, _> = users
        .select(id)
        .filter(login.eq(log))
        .filter(password_hash.eq(pass))
        .first(connection);
    match result {
        Ok(uid) => HttpResponse::Ok().json(LoginResponse {
            jwt_token: token_encode(uid),
        }),
        Err(DieselError::NotFound) => HttpResponse::Unauthorized().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[utoipa::path(
    responses((status = 200)),
    security(("jwt" = []))
)]
#[get("/auth/ping", wrap = "RequireAuth")]
async fn ping(user: UserIdentity) -> impl Responder {
    HttpResponse::Ok().json(user.id)
}
