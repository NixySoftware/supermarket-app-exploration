use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::layout::Layout;
use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::auth::signin::SignInPage;
use crate::pages::auth::signout::SignOutPage;
use crate::pages::home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/supermarket-web.css" />

        <Title text="Supermarket" />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Layout>
                <Routes>
                    <Route path="" view=HomePage />

                    <Route path="/auth/signin" view=SignInPage />
                    <Route path="/auth/signout" view=SignOutPage />
                </Routes>
            </Layout>
        </Router>
    }
}
