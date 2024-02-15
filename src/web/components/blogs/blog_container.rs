use leptos::*;
use web_sys::js_sys::eval;
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
            <TableOfContents class=""/>
            <script src="https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
	        <script src="https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.5/gsap.min.js">
            </script>
        </div>
    }
}

#[component]
fn TableOfContents(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    create_effect(move |_| {
        eval(r###"
            const headers = document.querySelectorAll('.blog-header1, .blog-header2')

            const handleItemClick = (event) => {
                const className = event.target.getAttribute('data-header-class')
                const headerElement = document.querySelector(`.${className}`)
                if (headerElement) {
                    headerElement.scrollIntoView({ behavior: 'smooth', block: 'start' })
                }
            }

            headers.forEach(header => {
                const listItem = document.createElement('li')
                const isSub = header.className.includes('blog-header2')
                const tailwind = isSub
                    ? 'pl-4'
                    : 'font-bold'
                listItem.className = `list list-none ${tailwind}` // tailwind styling
                const uniqueClassName = `c-${Math.random().toString(36).substring(7).trim()}`
                header.className = `${header.className} ${uniqueClassName}`
                const headerText = header.innerText
                listItem.innerText = headerText
                listItem.setAttribute('data-header-class', uniqueClassName)
                listItem.addEventListener('click', handleItemClick)
                document.querySelector('.table-of-contents').appendChild(listItem);
            })
        "###)
    });
    
    view! {
        <ul class=format!("table-of-contents {class}")></ul>
    }
}