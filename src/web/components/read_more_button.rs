use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn ReadMoreButton(
    children: Children,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = "{}")]
    style: &'static str,
    onclick: Callback<MouseEvent>,
) -> impl IntoView {
    view! {
        <button on:click=onclick class={format!("rounded-xl bg-white text-black font-main-bold px-3 py-2 w-fit hover:bg-gray-600 hover:outline hover:outline-gray-950 {}", class)} style={style}>
            {children()}
        </button>
    }
}
