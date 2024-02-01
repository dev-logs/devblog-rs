use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn BlogItemContainer(
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = "{}")]
    style: &'static str,
    onclick: Callback<MouseEvent>,
    children: Children,
) -> impl IntoView {
    view! {
        <div on:click=onclick class={format!("cursor-pointer flex flex-col m-5 {}", class)} style={style}>
            <div class="body flex flex-col">
                {children()}
            </div>
        </div>
    }
}
