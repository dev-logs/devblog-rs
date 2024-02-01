use leptos::*;

#[component]
pub fn BlogItemDescription(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div>
            <p class={format!("text-gray-400 font-light {}", class)}>{children()}</p>
        </div>
    }
}
