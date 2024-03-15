use leptos::*;

#[component]
pub fn AuthLayout(children: Children) -> impl IntoView {
    view! {
        <div class="mx-auto flex h-full w-full flex-col justify-center space-y-6 sm:w-[400px]">
            {children()}
        </div>
    }
}
