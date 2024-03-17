use leptos::*;
use supermarket_web_database::entities::prelude::Provider;
use supermarket_web_database::entities::provider;
use supermarket_web_database::sea_orm::{EntityTrait, QuerySelect};

use crate::context::use_database;

#[server(Providers)]
pub async fn providers() -> Result<Vec<provider::Model>, ServerFnError> {
    let database = use_database()?;

    // TODO: figure out if this is the best way to hide columns from the client

    Provider::find()
        .select_only()
        .columns([
            provider::Column::Id,
            provider::Column::CreatedAt,
            provider::Column::UpdatedAt,
            provider::Column::Name,
            provider::Column::Slug,
            provider::Column::Type,
        ])
        .all(&database)
        .await
        .map_err(ServerFnError::new)
}
