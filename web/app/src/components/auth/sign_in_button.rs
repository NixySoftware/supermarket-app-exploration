use leptos::*;
use log::info;
use supermarket_web_database::entities::provider;

use crate::components::shadcn_ui::button::{Button, ButtonVariant};

#[component]
pub fn SignInButton(provider: provider::Model) -> impl IntoView {
    let on_click = |_| info!("clicked");

    view! {
        <Button class="gap-3" variant=ButtonVariant::Outline on:click=on_click>
            // TODO: convert to kebab or snake case instead of lowercase
            <img class="h-5 w-5" src=format!("/images/icons/providers/{}.svg", format!("{:?}", &provider.r#type).to_lowercase()) alt="Icon" />
            <span>Sign in with {provider.name}</span>
        </Button>
    }
}
