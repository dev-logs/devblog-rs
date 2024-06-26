use leptos::*;

#[component]
pub fn BlogItemTitle(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <p class={format!("text-2xl font-main.jsx-bold {}", class)}>
            {children()}
        </p>
    }
}
