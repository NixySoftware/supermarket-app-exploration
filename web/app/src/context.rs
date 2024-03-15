use leptos::{use_context, ServerFnError};
use supermarket_web_database::sea_orm::DatabaseConnection;

pub fn use_database() -> Result<DatabaseConnection, ServerFnError> {
    use_context::<DatabaseConnection>().ok_or(ServerFnError::new(
        "Context is missing database connection.",
    ))
}
