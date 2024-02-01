use leptos::*;

#[component]
pub fn BlogHighLight(
    children: Children,
    #[prop(default = false)]
    bold: bool,
    #[prop(default = false)]
    background: bool,
    #[prop(default = false)]
    rounded: bool,
    #[prop(default = false)]
    italic: bool,
) -> impl IntoView {
    view! {
        <b class={
            format!(
                "{} {} {} {} {} p-1 font-main text-base",
                if rounded { "rounded-lg p-2 ml-1" } else { "" },
                if bold { "font-bold" } else { "" },
                if italic { "italic" } else { "" },
                if bold { "text-white" } else { "" },
                if background { "bg-blue-800" } else { "" },
            )
        }>
            {children()}
        </b>
    }
}
