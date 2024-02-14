use leptos::*;

#[component]
pub fn Avatar3(
    #[prop(default = 80)]
    width: i32,
    #[prop(default = 80)]
    height: i32,
) -> impl IntoView {
    view! {
<svg viewBox="0 0 36 36" fill="none" role="img" xmlns="http://www.w3.org/2000/svg" width=width height=height><mask id=":r2i:" maskUnits="userSpaceOnUse" x="0" y="0" width="36" height="36"><rect width="36" height="36" rx="72" fill="#FFFFFF"></rect></mask><g mask="url(#:r2i:)"><rect width="36" height="36" fill="#49007e"></rect><rect x="0" y="0" width="36" height="36" transform="translate(-4 8) rotate(88 18 18) scale(1.1)" fill="#ff7d10" rx="36"></rect><g transform="translate(0 4) rotate(8 18 18)"><path d="M13,20 a1,0.75 0 0,0 10,0" fill="#000000"></path><rect x="11" y="14" width="1.5" height="2" rx="1" stroke="none" fill="#000000"></rect><rect x="23" y="14" width="1.5" height="2" rx="1" stroke="none" fill="#000000"></rect></g></g></svg>
    }
}