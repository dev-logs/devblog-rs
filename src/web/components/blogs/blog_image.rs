use leptos::*;

#[component]
pub fn BlogImage(
    src: &'static str,
    #[prop(default = "")]
    alt: &'static str,
    #[prop(default = "")]
    caption: &'static str,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = "")]
    url: &'static str,
    #[prop(default = false)]
    spacing: bool
) -> impl IntoView {
    view! {
        <figure class={format!("{}{}", if spacing { "mt-5 " } else { "" }, class)}>
            <img class="h-auto max-w-full rounded-lg" src={src} alt={alt}/>
            <figcaption class="mt-2 text-sm text-center text-gray-500">
                {if !url.is_empty() {
                    view! { <a class="text-gray-500 text-base" href={url}>move || caption</a> }
                } else {
                    view! {<a class="text-gray-500 -bash">{move || caption}</a>}
                }}
            </figcaption>
        </figure>
    }
}
