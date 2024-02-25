use leptos::*;
use leptos::logging::log;
use web_sys::MouseEvent;
use crate::core_services::web_di::*;
use crate::entities::blog::Blog;
use crate::services::like::service::LikeBlogParam;
use crate::web::local_storage::user::UserStorage;
use crate::services::base::service::*;

#[component]
pub fn ThumbUpRive(
    #[prop()]
    blog: Blog
) -> impl IntoView {
    let (how_many_like_total, set_total_like_count) = create_signal::<u32>(0);
    let (how_many_new_like, set_new_like_count) = create_signal::<u32>(0);

    let on_like = {
        let how_many_like_total = how_many_like_total.clone();
        let set_total_like_count = set_total_like_count.clone();
        let how_many_new_like = how_many_new_like.clone();
        let set_new_like_count = set_new_like_count.clone();
        Callback::new(move |e: MouseEvent| {
            set_total_like_count(how_many_like_total.get_untracked() + 1);
            set_new_like_count(how_many_new_like.get_untracked() + 1);
        })
    };

    let perform_like = {
        let blog = blog.clone();
        let how_many_new_like = how_many_new_like.clone();
        let set_new_like_count = set_new_like_count.clone();
        create_action(move |_: &()| {
            let how_many_new_like = how_many_new_like.clone();
            let blog = blog.clone();
            async move {
                let like_count = how_many_new_like.get_untracked();
                set_new_like_count(0);
                let user_storage = UserStorage::new();
                let user = user_storage.data.expect("You must signin first");
                let service = WebInjector::service_injector().get_like_blog_service();
                let result = service.execute(LikeBlogParam {
                    blog_title: blog.title.clone(),
                    display_name: user.display_name.clone(),
                    count: like_count
                }).await;
                log!("Performed {:?}", result);
                ()
            }
        })
    };

    let on_like_confirm = {
        Callback::new(move |e: MouseEvent| {
            perform_like.dispatch(());
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
                <rive-thumb-up id="riveThumbUpLike" class="block w-full h-full" on:LikeEvent=on_like on:LikeConfirmEvent=on_like_confirm likeCount=10></rive-thumb-up>
                <rive-text id="riveTextLike" class="block w-full h-full" text={move || {format!("{} likes", how_many_like_total.get().to_string())}}></rive-text>
            </div>
            <div class="flex flex-row row-span-2">
                <rive-emoji-face-love id="riveEmojiView" class="block w-full h-full m-1"></rive-emoji-face-love>
                <rive-text id="riveTextView" class="block w-full h-full" text={move || {format!("{} views", how_many_like_total.get().to_string())}}></rive-text>
            </div>
            <script src="/assets/js/rive/index.js"></script>
        </div>
    }
}
