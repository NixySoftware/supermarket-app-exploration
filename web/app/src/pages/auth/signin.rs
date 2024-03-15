use leptos::*;

use crate::{
    actions::provider::providers,
    components::auth::{header::AuthHeader, layout::AuthLayout},
};

#[component]
pub fn SignInPage() -> impl IntoView {
    let list_resource = create_resource(|| (), |_| async move { providers().await });

    view! {
       <AuthLayout>
            <AuthHeader>Sign in</AuthHeader>

            <div class="grid gap-6">

            </div>
       </AuthLayout>
    }
}
