use leptos::*;

#[component]
pub fn Avatar4(
    #[prop(default = 80)]
    width: i32,
    #[prop(default = 80)]
    height: i32,
) -> impl IntoView {
    view! {
        <svg viewBox="0 0 36 36" fill="none" role="img" xmlns="http://www.w3.org/2000/svg" width=width height=height><mask id=":r1u:" maskUnits="userSpaceOnUse" x="0" y="0" width="36" height="36"><rect width="36" height="36" rx="72" fill="#FFFFFF"></rect></mask><g mask="url(#:r1u:)"><rect width="36" height="36" fill="#ffb238"></rect><rect x="0" y="0" width="36" height="36" transform="translate(3 3) rotate(181 18 18) scale(1.1)" fill="#49007e" rx="36"></rect><g transform="translate(-5 -2) rotate(-1 18 18)"><path d="M15 20c2 1 4 1 6 0" stroke="#FFFFFF" fill="none" stroke-linecap="round"></path><rect x="13" y="14" width="1.5" height="2" rx="1" stroke="none" fill="#FFFFFF"></rect><rect x="21" y="14" width="1.5" height="2" rx="1" stroke="none" fill="#FFFFFF"></rect></g></g></svg>
    }
}