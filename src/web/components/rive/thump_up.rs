use leptos::*;
use web_sys::MouseEvent;
use crate::entities::blog::Blog;

#[component]
pub fn ThumbUpRive(
    #[prop()]
    blog: Blog
) -> impl IntoView {
    let (how_many_like, set_like_count) = create_signal(1);
    let on_like = {
        let how_many_like = how_many_like.clone();
        let set_like_count = set_like_count.clone();
        Callback::new(move |e: MouseEvent| {
            set_like_count(how_many_like.get() + 1);
        })
    };

    let author_name = blog.author.display_name.clone().unwrap();
    view! {
        <div class="grid grid-rows-10 divide-y divide-gray-700 mx-5 h-80 h-72 border border-gray-700 mt-10 rounded-xl max-w-64">
            <div class="row-span-6 flex-col justify-between flex p-2 pl-5">
                <div>
                   <p class="font-main text-lg">{blog.title.clone()}</p>
                   <p class="font-main-bold text-md mt-2">{author_name}</p>
                </div>
                <p class="font-main text-gray-600 mb-2 text-sm">1 min read</p>
            </div>
            <div class="flex flex-row row-span-2">
                <rive-thumb-up id="riveThumbUpLike" class="block w-full h-full" on:LikeEvent=on_like likeCount=10></rive-thumb-up>
                <rive-text id="riveTextLike" class="block w-full h-full" text={move || {format!("{} likes", how_many_like.get().to_string())}}></rive-text>
            </div>
            <div class="flex flex-row row-span-2">
                <rive-emoji-face-love id="riveEmojiView" class="block w-full h-full m-1"></rive-emoji-face-love>
                <rive-text id="riveTextView" class="block w-full h-full" text={move || {format!("{} views", how_many_like.get().to_string())}}></rive-text>
            </div>
            <script src="/assets/js/rive/index.js"></script>
        </div>
    }
}
