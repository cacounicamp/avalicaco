use std::{
    future::{self, Future, Ready},
    pin::Pin,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::ContentType,
    HttpMessage, HttpResponse,
};
use chrono::{Duration, Local};
pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct UserIdentity {
    pub id: i32,
}

impl actix_web::FromRequest for UserIdentity {
    // works
    type Error = ::actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(
            match <actix_web::HttpRequest as actix_web::HttpMessage>::extensions(req)
                .get::<UserIdentity>()
            {
                Some(user_identity) => Ok(user_identity.clone()),
                None => Err(actix_web::error::ErrorInternalServerError(
                    "Internal server error",
                )),
            },
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
    exp: i64,
}

pub fn token_encode(id: i32) -> String {
    let claims = Claims {
        id,
        exp: (Local::now() + Duration::hours(1)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}
const HEADER_AUTHORIZATION: &str = "Authorization";

pub struct RequireAuth;

impl<S> Transform<S, ServiceRequest> for RequireAuth
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Transform = ApiKeyMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ApiKeyMiddleware { service }))
    }
}

pub struct ApiKeyMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for ApiKeyMiddleware<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let response = |req: ServiceRequest, response: HttpResponse| -> Self::Future {
            Box::pin(async { Ok(req.into_response(response)) })
        };
        let Some(header_token) = req.headers().get(HEADER_AUTHORIZATION) else {
            return response(
                req,
                HttpResponse::Unauthorized()
                    .content_type(ContentType::plaintext())
                    .body(String::from("missing api key")),
            );
        };
        let mut token = header_token.to_str().unwrap();
        token = token.strip_prefix("Bearer").unwrap_or(token).trim();
        let decode_result = decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
        let Ok(data) = decode_result else {
            return response(
                req,
                HttpResponse::Unauthorized()
                    .content_type(ContentType::plaintext())
                    .body(String::from("incorrect api key")),
            );
        };
        req.extensions_mut()
            .insert(UserIdentity { id: data.claims.id });
        Box::pin(self.service.call(req))
    }
}
