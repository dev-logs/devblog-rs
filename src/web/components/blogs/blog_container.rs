use leptos::*;
use crate::web::components::blogs::blog_header::BlogHeader;
use crate::web::discussion::Discussion;

#[component]
pub fn BlogContainer(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="w-full flex flex-row justify-center items-center">
            <link rel="stylesheet" href="https://unpkg.com/prismjs@1.29.0/themes/prism-twilight.css" crossorigin="anonymous" referrerpolicy="no-referrer" />
            <article class="mt-8 prose prose-lg">
                <div class={format!("flex flex-col items-start justify-start {class}")}>
                    {children()}
                    <BlogHeader>Discussions</BlogHeader>
                    <Discussion/>
                </div>
            </article>
            <script src="https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
	        <script src="https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.5/gsap.min.js">
            </script>
        </div>
    }
}