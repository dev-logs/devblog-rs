use leptos::*;

#[component]
pub fn ArrowDown(
    #[prop(default = 1)]
    width: i32,
    #[prop(default = 1)]
    height: i32,
    #[prop(default = "#000")]
    color: &'static str
) -> impl IntoView {
    view! {
        <svg width={format!("{}em", width)} height={format!("{}em", height)} viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
            <g fill="none" stroke={color} strokeLinecap="round" strokeLinejoin="round" strokeWidth="4">
                <path d="M24 42V6"></path>
                <path d="M36 30L24 42L12 30"></path>
            </g>
        </svg>
    }
}