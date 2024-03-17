use leptos::*;

use crate::{
    actions::provider::providers,
    components::auth::{header::AuthHeader, layout::AuthLayout, sign_in_button::SignInButton},
};

#[component]
pub fn SignInPage() -> impl IntoView {
    let list_resource = create_resource(|| (), |_| async move { providers().await.unwrap() });

    view! {
       <AuthLayout>
            <AuthHeader>Sign in</AuthHeader>

            <Transition fallback=move || {
                view! { <span>"Loading..."</span> }
            }>
                <div class="grid gap-6">
                    <For
                        each=move || list_resource.get().unwrap_or_default()
                        key=|provider| provider.id
                        children=move |provider| {
                            view! {
                                <SignInButton provider=provider />
                            }
                        }
                    />
                </div>
            </Transition>
       </AuthLayout>
    }
}
