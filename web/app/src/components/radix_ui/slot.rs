use leptos::*;

// TODO: support slottable

#[component]
pub fn Slot(
    #[prop(attrs)] attributes: Vec<(&'static str, Attribute)>,
    children: Children,
) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| child)
        .collect_view();

    view! {}
}
