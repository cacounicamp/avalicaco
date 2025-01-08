use super::{schema::evaluations, schema::evaluations::dsl, NewEvaluation, UpdateEvaluation};
use diesel::{prelude::*, result::Error as DieselError};

pub fn evaluation_create(
    connection: &mut PgConnection,
    evaluation: NewEvaluation,
) -> Result<usize, DieselError> {
    diesel::insert_into(evaluations::table)
        .values(&evaluation)
        .execute(connection)
}
pub fn evaluation_update(
    connection: &mut PgConnection,
    evaluation_id: i32,
    update: UpdateEvaluation,
) -> Result<usize, DieselError> {
    diesel::update(dsl::evaluations.filter(dsl::id.eq(evaluation_id)))
        .set((dsl::title.eq(update.title), dsl::date.eq(update.date)))
        .execute(connection)
}
pub fn evaluation_delete(
    connection: &mut PgConnection,
    evaluation_id: i32,
) -> Result<usize, DieselError> {
    diesel::delete(dsl::evaluations.filter(dsl::id.eq(evaluation_id))).execute(connection)
}
