use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn ThumbUpRive() -> impl IntoView {
    let (how_many_like, set_like_count) = create_signal(1);
    let on_like = {
        let how_many_like = how_many_like.clone();
        let set_like_count = set_like_count.clone();
        Callback::new(move |e: MouseEvent| {
            set_like_count(how_many_like.get() + 1);
        })
    };

    view! {
        <div class="grid grid-cols-3 mx-5 h-44">
            <div class="flex flex-row">
                <rive-thumb-up id="riveThumbUpLike" class="block w-full h-full" on:LikeEvent=on_like likeCount=10></rive-thumb-up>
                <rive-text id="riveTextLike" class="block w-full h-full" text={move || {how_many_like.get().to_string()}}></rive-text>
            </div>
            <div>
                <rive-emoji-face-love id="riveEmojiView" class="block w-full h-full"></rive-emoji-face-love>
            </div>
            <div></div>

            <script src="/assets/js/rive/index.js"></script>
        </div>
    }
}
