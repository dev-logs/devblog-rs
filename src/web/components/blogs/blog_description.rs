use leptos::*;

#[component]
pub fn BlogDescription(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div>
            <p class={format!("text-gray-400 font-light font-main.jsx text-xl {}", class)}>{children()}</p>
        </div>
    }
}
