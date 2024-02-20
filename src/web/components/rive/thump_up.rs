use leptos::*;
use leptos::logging::log;
use web_sys::js_sys::eval;
use web_sys::MouseEvent;

#[component]
pub fn ThumbUpRive() -> impl IntoView {
    let on_like = Callback::new(|e: MouseEvent| {
        log!("on like");
    });

    view! {
        <div class="">
            <thumb-up id="thumb" class="block w-full h-96" on:LikeEvent=on_like></thumb-up>
            <script src="/assets/js/rive/index.js"></script>
        </div>
    }
}
