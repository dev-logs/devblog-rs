use leptos::*;

#[component]
pub fn BlogTitle(
    #[prop(default = "flex flex-row")]
    class: &'static str,
    #[prop()]
    children: Children
) -> impl IntoView {
    return view! {
        <div class={format!("{class} m-16 rounded-3xl border-gray-700 p-16 border-2")} style:background-color="#070F2B">
            {children()}
        </div>
    }
}
