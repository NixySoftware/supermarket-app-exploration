use leptos::*;

use crate::components::auth::{header::AuthHeader, layout::AuthLayout};

#[component]
pub fn SignOutPage() -> impl IntoView {
    view! {
       <AuthLayout>
            <AuthHeader>Sign out</AuthHeader>

       </AuthLayout>
    }
}
