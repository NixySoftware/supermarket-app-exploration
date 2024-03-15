use leptos::*;
use supermarket_web_database::entities::prelude::Provider;
use supermarket_web_database::entities::provider;
use supermarket_web_database::sea_orm::EntityTrait;

use crate::context::use_database;

#[server(Providers)]
pub async fn providers() -> Result<Vec<provider::Model>, ServerFnError> {
    let database = use_database()?;

    let providers = Provider::find().all(&database).await?;

    println!("Providers: {:#?}", providers);

    Ok(providers)
}
