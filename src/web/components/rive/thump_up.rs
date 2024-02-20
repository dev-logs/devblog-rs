use leptos::*;
use leptos::logging::log;
use web_sys::js_sys::eval;

#[component]
pub fn ThumbUpRive() -> impl IntoView {
    create_effect(move |_| {
        eval(r###"

        "###)
    });

    view! {
        <div class="">
            <thumb-up id="thumb" class="block w-full h-96"></thumb-up>
            <script src="/assets/js/rive/index.js"></script>
        </div>
    }
}
