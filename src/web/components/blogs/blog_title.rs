use leptos::*;

#[component]
pub fn BlogTitle(
    #[prop(default = "flex flex-row")]
    class: &'static str,
    #[prop()]
    children: Children
) -> impl IntoView {
    return view! {
        <div class=format!("{class} justify-center items-center w-screen overflow-hidden")>
            <div class="max-w-screen-3xl" style:height="50svh">
                {children()}
            </div>
        </div>
    }
}
