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
        <div class={format!("{}{}", if !spacing { "mt-10 " } else { "" }, class)}>
            <p class="text-gray-400 font-bold font-main text-xl w-full">{children()}</p>
        </div>
    }
}
