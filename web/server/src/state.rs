use axum::extract::FromRef;
use leptos::LeptosOptions;
use supermarket_web_database::sea_orm::DatabaseConnection;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub database: DatabaseConnection,
}
