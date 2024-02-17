use leptos::*;

#[component]
pub fn BlogHeader(
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = false)]
    spacing: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("{}{}", if spacing { "mt-8 " } else { "" }, class)}>
            <p class="blog-header1 text-gray-200 font-bold font-main prose-sm text-3xl w-full">{children()}</p>
        </div>
    }
}
