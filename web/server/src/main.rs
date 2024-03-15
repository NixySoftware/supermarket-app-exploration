mod auth;
mod database;
mod fileserv;
mod state;

use axum::{
    body::Body, extract::State, http::Request, response::IntoResponse, routing::get, Router,
};
use axum_login::tower_sessions::{cookie::SameSite, Expiry, MemoryStore, SessionManagerLayer};
use axum_login::AuthManagerLayerBuilder;
use dotenvy::dotenv;
use leptos::*;
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use supermarket_web_app::App;

use crate::auth::Backend;
use crate::database::get_database_connection;
use crate::fileserv::file_and_error_handler;
use crate::state::AppState;

fn provide_context_from_app_state(app_state: AppState) {
    provide_context(app_state.database.clone());
}

async fn server_fns_handler(
    State(app_state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context_from_app_state(app_state.clone());
        },
        req,
    )
    .await
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let database = get_database_connection().await.unwrap();

    let session_store = MemoryStore::default();
    let session_layer: SessionManagerLayer<MemoryStore> = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(
            axum_login::tower_sessions::cookie::time::Duration::days(1),
        ))
        .with_http_only(true)
        .with_secure(true)
        .with_same_site(SameSite::Lax);

    let backend = Backend::new(database.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app_state = AppState {
        leptos_options,
        database,
    };
    let app_state_context = app_state.clone();

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fns_handler).post(server_fns_handler),
        )
        .leptos_routes_with_context(
            &app_state,
            routes,
            move || {
                provide_context_from_app_state(app_state_context.clone());
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
