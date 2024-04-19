use leptos::*;

#[component]
pub fn BlogHeader2(
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = false)]
    spacing: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("{}{}", if spacing { "mt-4 " } else { "" }, class)}>
            <p class="blog-header2 text-gray-200 font-main.jsx font-main.jsx-bold text-xl w-full">{children()}</p>
        </div>
    }
}
