use leptos::*;

#[component]
pub fn BlogContainer(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="w-full flex flex-row justify-center items-center">
            <link rel="stylesheet" href="https://unpkg.com/prismjs@1.29.0/themes/prism-twilight.css" crossorigin="anonymous" referrerpolicy="no-referrer" />
            <div class={format!("flex flex-col items-start justify-start {class}")}>
                <article class="mt-8 prose prose-lg">
                    {children()}
                </article>
            </div>
            <script src="https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
	        <script src="https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
        </div>
    }
}