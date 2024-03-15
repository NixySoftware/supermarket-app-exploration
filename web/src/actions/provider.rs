use crate::entities::prelude::Provider;
use crate::entities::provider;
use leptos::*;

#[server(Providers)]
pub async fn providers() -> Result<Vec<provider::Model>, ServerFnError> {
    use crate::state::use_app_state;
    use sea_orm::EntityTrait;

    let app_state = use_app_state()?;

    println!("{:#?}", app_state);

    let providers = Provider::find().all(&app_state.database).await?;

    println!("Providers: {:#?}", providers);

    Ok(providers)
}
