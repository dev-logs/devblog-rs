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
        <div class={format!("{}{}", if spacing { "mt-4 " } else { "" }, class)}>
            <p class="text-gray-500 font-bold font-main text-lg w-full">{children()}</p>
        </div>
    }
}
