use leptos::*;

#[component]
pub fn BlogBody(
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = false)]
    newline: bool,
    #[prop(default = false)]
    spacing: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class={format!(
            "{} {} {}",
            class,
            if newline { "mt-2" } else { "" },
            if !spacing { "mt-6" } else { "" },
        )}>
            <p class="text-gray-400 font-thin text-base w-full font-main leading-8">{children()}</p>
        </div>
    }
}
