use axum::extract::FromRef;
use leptos::{use_context, LeptosOptions, ServerFnError};
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub database: DatabaseConnection,
}

pub fn use_app_state() -> Result<AppState, ServerFnError> {
    use_context::<AppState>().ok_or(ServerFnError::new("Missing app context."))
}
