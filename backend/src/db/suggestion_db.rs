use crate::db::{
    evaluation_db::{evaluation_create, evaluation_delete, evaluation_update},
    prelude::*,
    schema::suggestion::dsl,
    NewEvaluation, SuggestionDetails, SuggestionModel, SuggestionSelect, UpdateEvaluation,
};

pub fn suggestion_select_by_id(
    connection: &mut PgConnection,
    id: i32,
) -> Result<SuggestionSelect, DieselError> {
    dsl::suggestion
        .select(SuggestionSelect::as_select())
        .filter(dsl::id.eq(id))
        .first(connection)
}

fn suggestion_delete(connection: &mut PgConnection, id: i32) -> Result<usize, DieselError> {
    diesel::delete(dsl::suggestion.filter(dsl::id.eq(id))).execute(connection)
}

pub fn suggestion_apply(connection: &mut PgConnection, sug: SuggestionModel) {
    suggestion_delete(connection, sug.id).expect("todo");
    match sug.details {
        SuggestionDetails::Create { title, date, class } => evaluation_create(
            connection,
            NewEvaluation {
                title,
                date: date.naive_utc(),
                class,
            },
        ),
        SuggestionDetails::Update {
            evaluation_id,
            title,
            date,
        } => evaluation_update(
            connection,
            evaluation_id,
            UpdateEvaluation {
                date: date.naive_utc(),
                title,
            },
        ),
        SuggestionDetails::Delete { evaluation_id } => evaluation_delete(connection, evaluation_id),
    }
    .expect("todo");
}
