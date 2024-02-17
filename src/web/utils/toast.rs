use leptos::leptos_dom::log;
use web_sys::js_sys::eval;

pub fn show_toast(html: String) {
    let script = format!(r#"
        const toastContainer = document.createElement('div');
        toastContainer.style.position = "fixed"
        toastContainer.style.top = "0px"
        toastContainer.style.zIndex = "100"
        toastContainer.style.right = "0px"
        toastContainer.className = 'toast-container'
        document.body.appendChild(toastContainer)

        toastContainer.innerHTML = `{html}`;
        gsap.set(toastContainer, {{x: '100%', y: '20px', opacity: 0}});
        gsap.to(toastContainer, {{x: '0%', opacity: 1, duration: 0.5, ease: 'power2.out'}});

        setTimeout(() => {{
            gsap.to(
                toastContainer,
                {{
                    x: '100%',
                    opacity: 0,
                    duration: 0.5,
                    ease: 'power2.in',
                    onComplete: () => {{
                        toastContainer.remove();
                    }}
                }}
            )
        }}, 3000);
    "#);

    log!("Executing script {script}");
    eval(script.as_str()).expect("TODO: panic message");
}

pub fn show_welcome_toast(display_name: &str) {
    // HTML content
    let html_content = format!(r###"
        <div id='toast-message-cta' class='w-full max-w-xs p-4 text-gray-500 bg-white rounded-lg shadow dark:bg-gray-800 dark:text-gray-400' role='alert'>
            <div class="flex'>
                <img class='w-8 h-8 rounded-full' src='/assets/images/ic_devlog_transparent.png' alt='logo'/>
                <div class='ms-3 text-sm font-normal'>
                    <span class='mb-1 text-sm font-semibold text-gray-900 dark:text-gray-200'>devlog.studio</span>
                    <div class='mb-2 text-sm font-normal'>Hi {display_name}, thanks for joining our community</div>
                    <a href="#" class='inline-flex px-2.5 py-1.5 text-xs font-medium text-center text-gray-200 bg-blue-600 rounded-lg hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-500 dark:hover:bg-blue-600 dark:focus:ring-blue-800'>Reply</a>
                </div>
            </div>
        </div>
    "###).replace("'", "\"");

    log!("Append {html_content}");

    show_toast(html_content.to_string());
}