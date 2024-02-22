use leptos::*;
use web_sys::js_sys::eval;
use crate::entities::blog::Blog;
use crate::web::components::blogs::blog_header::BlogHeader;
use crate::web::components::rive::thump_up::ThumbUpRive;
use crate::web::discussion::Discussion;

#[component]
pub fn BlogContainer<E, F>(
    #[prop(default = "")]
    class: &'static str,
    children: Children,
    header: F,
    #[prop()]
    blog: Blog
) -> impl IntoView
    where E: IntoView, F: Fn() -> E + 'static
{
    view! {
        <div class="grid grid-cols-10 justify-start bg-gray-950">
            // {header()}
            <link rel="stylesheet" href="https://unpkg.com/prismjs@1.29.0/themes/prism-twilight.css" crossorigin="anonymous" referrerpolicy="no-referrer" />
            <div class="sticky top-0 right-0 h-screen col-span-3 flex flex-row w-full items-start justify-center">
                <ThumbUpRive blog=blog/>
            </div>
            <div class="overflow-auto col-span-4">
                <article class="mt-8 prose prose-lg flex flex-col h-full items-start justify-start overflow-auto">
                    {children()}
                    <BlogHeader>Discussions</BlogHeader>
                    <Discussion/>
                </article>
            </div>
            <div class="sticky top-0 right-0 pt-10 justify-end items-end text-start h-screen pl-12 overflow-auto col-span-3">
                <TableOfContents class=""/>
            </div>
            <script src="https://unpkg.com/prismjs@1.29.0/components/prism-core.min.js" crossorigin="anonymous" referrerpolicy="no-referrer"></script>
	        <script src="https://unpkg.com/prismjs@1.29.0/plugins/autoloader/prism-autoloader.min.js"></script>
            <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.5/gsap.min.js"></script>
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

            const handleItemClick = () => (event) => {
                const className = event.target.getAttribute('data-header-class')
                const headerElement = document.querySelector(`.${className}`)
                if (headerElement) {
                    headerElement.scrollIntoView()
                }
            }

            headers.forEach((header, index) => {
                const listItem = document.createElement('button')
                const isSub = header.className.includes('blog-header2')
                const tailwind = isSub
                    ? 'text-sm button pl-4 text-gray-400 z-20'
                    : 'text-sm text-gray-200 pl-2 z-20'
                listItem.className = `list list-none text-start ${tailwind}` // tailwind styling
                const uniqueClassName = `c-${index}`
                header.className = `${header.className} ${uniqueClassName}`
                const headerText = header.innerText
                listItem.innerText = headerText
                listItem.setAttribute('data-header-class', uniqueClassName)
                listItem.addEventListener('click', handleItemClick())
                const item = document.createElement('li')
                item.className = `${uniqueClassName}-toc highlight-target z-10 w-fit py-1`
                item.appendChild(listItem)
                header.setAttribute('tocClass', `${uniqueClassName}-toc`)
                document.querySelector('.table-of-contents').appendChild(item);
            })

            const handleIntersection = (entries, observer) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        const element = document.querySelector(`.${entry.target.getAttribute('tocClass')}`)
                        const bounding = element.getBoundingClientRect()
                        const tableOfContents = document.querySelector('.table-of-contents').parentElement.parentElement
                        const parentBounding = tableOfContents.getBoundingClientRect()
                        if (bounding.top > parentBounding.height || (element.offsetTop - tableOfContents.scrollTop) < 0) {
                            tableOfContents.scrollTo({top: element.offsetTop})
                        }

                        const selectorElement = document.querySelector('.selector')
                        const selectorInitialBound = selectorElement.savedBounding || selectorElement.getBoundingClientRect()
                        selectorElement.savedBounding = selectorInitialBound

                        gsap.to(selectorElement, {
                            y: element.offsetTop,
                            width: bounding.width + 20,
                            height: bounding.height,
                            duration:  0.1,
                            ease: 'bounce.inOut'
                        })
                    }
                })
           }

            const observer = new IntersectionObserver(handleIntersection, {
                root: null,
                rootMargin: '0px 0px -80% 0px', // Adjust the top margin as needed
                threshold: 0
            });

            headers.forEach((item) => observer.observe(item))
        "###)
    });
    
    view! {
        <div class=format!("relative p-2 {class}")>
            <div class="flex flex-col">
                <p class="text-lg mb-4 pl-1">Contents</p>
                <ul class="table-of-contents box z-20 h-1/2 overflow-auto"/>
            </div>
            <div class="selector absolute top-0 left-2 w-40 h-8 rounded-lg border z-10 bg-zinc-900"/>
        </div>
    }
}