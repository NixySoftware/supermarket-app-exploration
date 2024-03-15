use leptos::*;

#[component]
pub fn AuthHeader(children: Children) -> impl IntoView {
    view! {
        <div class="mt-[-64px] flex flex-col space-y-2 text-center">
            <h1 class="text-2xl font-semibold tracking-tight">{children()}</h1>
        </div>
    }
}
