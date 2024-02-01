use leptos::*;

#[component]
pub fn StandardSizeWrapper<'a>(
    #[prop(default = "")]
    class_name: &'a str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!("ml-auto mr-auto justify-self-auto w-full max-w-screen-lg {}", class_name)}>
            {children()}
        </div>
    }
}
