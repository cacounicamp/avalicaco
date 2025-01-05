use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
pub mod tag {
    pub const AUTH: &str = "Authentication";
    pub const USERS: &str = "Users";
    pub const EVALUATIONS: &str = "Evaluations";
}
#[derive(OpenApi)]
#[openapi(
	tags(
		(name = tag::AUTH),
		(name = tag::USERS),
		(name = tag::EVALUATIONS),
	),
	modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
