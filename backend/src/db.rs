pub mod schema;
pub mod models;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



pub fn create_evaluation(conn: &mut PgConnection, evaluation: NewEvaluation) -> Evaluation {
    use schema::evaluations;


    diesel::insert_into(evaluations::table)
        .values(&evaluation)
        .returning(Evaluation::as_returning())
        .get_result(conn)
        .expect("Error saving new evaluation")
}