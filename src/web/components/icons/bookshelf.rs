use leptos::*;

#[component]
pub fn BookShelf(
    #[prop(default = 1)]
    width: i32,
    #[prop(default = 1)]
    height: i32,
    #[prop(default = "#000")]
    color: &'static str
) -> impl IntoView {
    view! {
        <svg width={format!("{}em", width)} height={format!("{}em", height)} viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg">
            <path fill={color} stroke={color} strokeLinecap="round" strokeLinejoin="round" strokeWidth="4" d="M5 6h34s4 2 4 7s-4 7-4 7H5s4-2 4-7s-4-7-4-7m38 22H9s-4 2-4 7s4 7 4 7h34s-4-2-4-7s4-7 4-7"></path>
        </svg>
    }
}