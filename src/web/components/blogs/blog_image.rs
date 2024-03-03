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
    spacing: bool,
    #[prop(default = false)]
    border: bool,
    #[prop(default = false)]
    blended: bool
) -> impl IntoView {
    view! {
        <figure class={format!("relative h-full {}{}{}", if spacing { "mt-5 " } else { "" }, if border {"border p-2 border-2 border-gray-900 rounded-xl flex items-center"} else { "" }, class)}>
            <img class="h-full rounded-lg object-cover w-full" src={src} alt={alt}/>
            {
                if blended {
                    view! {<div class="absolute inset-x-0 bottom-0 b w-full h-40 bg-gradient-to-t from-gray-950"></div>}
                } else {
                    view!{<div></div>}
                }
            }
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
