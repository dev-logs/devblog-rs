use leptos::*;

#[component]
pub fn BlogHeader3(
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = false)]
    spacing: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("{}{}", if spacing { "mt-12 " } else { "mt-8" }, class)}>
            <p class="text-gray-200 font-main.jsx-bold font-main.jsx text-lg w-full">{children()}</p>
        </div>
    }
}
