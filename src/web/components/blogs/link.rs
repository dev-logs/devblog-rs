use leptos::*;

#[component]
pub fn BlogLink(
    href: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a href={href} class="font-medium text-blue-600 underline dark:text-blue-500 hover:no-underline">{children()}</a>
    }
}
