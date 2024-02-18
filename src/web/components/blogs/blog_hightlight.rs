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
    #[prop(default = false)]
    border: bool
) -> impl IntoView {
    view! {
        <b class={
            format!(
                "{} {} {} {} {} {} p-1 font-main",
                if rounded { "rounded-lg p-2 ml-1" } else { "" },
                if bold { "font-main-bold" } else { "" },
                if italic { "italic" } else { "" },
                if bold { "text-gray-200" } else { "" },
                if background { "bg-blue-800" } else { "" },
                if border { "border border-white rounded-xl" } else { "" },
            )
        }>
            {children()}
        </b>
    }
}
