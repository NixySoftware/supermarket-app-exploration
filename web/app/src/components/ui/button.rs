use leptos::{html::div, *};

#[component]
fn Slot() -> impl IntoView {
    view! {
        <div>Test</div>
    }
}

#[component]
pub fn Button(#[prop(optional)] as_child: bool) -> impl IntoView {
    let Comp = if as_child { Slot } else { div };

    view! {}
}
