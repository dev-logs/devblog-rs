use leptos::*;
use leptos::logging::log;
use web_sys::MouseEvent;
use crate::core_services::web_di::*;
use crate::entities::blog;
use crate::entities::blog::Blog;
use crate::services::like::perform::service::LikeBlogParam;
use crate::web::local_storage::user::UserStorage;
use crate::services::base::service::*;
use crate::services::blog_detail::min_read::service::Params;
use crate::services::blog_detail::read;
use crate::services::like::counting::service::CountBlogLikeParams;
use crate::services::blog_detail::count_read::service::Params as CountReadParams;

#[component]
pub fn ThumbUpRive(
    #[prop()]
    blog: Blog,
    #[prop(default = "")]
    class: &'static str,
    #[prop(default = false)]
    is_mobile: bool
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

    let fetch_likes = {
        let blog = blog.clone();
        let set_total_like_count = set_total_like_count.clone();
        create_action(move |e: &()| {
            let blog = blog.clone();

            async move {
                let service = WebInjector::service_injector().get_count_blog_like_service();
                let result = service.execute(CountBlogLikeParams {
                    title: blog.title.clone()
                }).await;

                set_total_like_count(result.unwrap());
                ()
            }
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
                let user = user_storage.data;

                let service = WebInjector::service_injector().get_like_blog_service();
                let result = service.execute(LikeBlogParam {
                    blog_title: blog.title.clone(),
                    display_name: user.map(|u| u.display_name),
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

    let min_read_action = create_action(|e: &()| async {
        WebInjector::service_injector().get_count_read_minutes_service().execute(Params {}).await.unwrap()
    });

    let count_read_action = {
        let blog = blog.clone();
        create_action(move |e: &()| {
            let blog = blog.clone();
            async move {
                WebInjector::service_injector().get_count_read_service().execute(CountReadParams {
                    blog_title: blog.title.clone()
                }).await.unwrap()
            }
        })
    };

    let mark_read_action = {
        let blog = blog.clone();
        create_action(move |e: &()| {
            let blog = blog.clone();
            async move {
                let user = UserStorage::new().get().map(|it| it.clone());
                WebInjector::service_injector().get_mark_read_service_with_duration().execute(read::mark_read_service::Params {
                    blog_title: blog.title.clone(),
                    user
                }).await.unwrap()
            }
        })
    };

    create_effect(move |e| {
        min_read_action.dispatch(());
        mark_read_action.dispatch(());
    });

    let author_name = blog.author.display_name.clone().unwrap();

    if is_mobile {
        return view! {
            <div class="flex flex-row h-14 w-full bg-neutral-950 shadow shadow-gray-300 shadow-lg pb-2">
                <rive-thumb-up id="riveThumbUpLike" class="block w-16 h-full ml-4" on:LikeEvent=on_like on:LikeConfirmEvent=on_like_confirm likeCount=10></rive-thumb-up>
                <rive-text id="riveTextLike" class="block w-32 h-full pr-10" on:LoadComplete={move |e: MouseEvent| {fetch_likes.dispatch(())}} text={move || {format!("{} likes", how_many_like_total.get().to_string())}}></rive-text>
            </div>
        }
    }

    view! {
        <div class="grid grid-rows-10 divide-y divide-gray-700 mx-5 h-80 h-72 border border-gray-700 mt-10 rounded-xl max-w-64">
            <div class="row-span-6 flex-col justify-between flex p-2 pl-5">
                <div>
                   <p class="font-main text-lg">{blog.title.clone()}</p>
                   <p class="font-main-bold text-md mt-2">{author_name}</p>
                </div>
                <p class="font-main text-gray-600 mb-2 text-sm">{move || {format!("{} minutes", min_read_action.value().get().as_ref().map(|it| it.to_string()).unwrap_or("...".to_owned()))}}</p>
            </div>
            <div class="flex flex-row row-span-2">
                <rive-thumb-up id="riveThumbUpLike" class="block w-full h-full" on:LikeEvent=on_like on:LikeConfirmEvent=on_like_confirm likeCount=10></rive-thumb-up>
                <rive-text id="riveTextLike" class="block w-full h-full" text={move || {format!("{} likes", how_many_like_total.get().to_string())}}></rive-text>
            </div>
            <div class="flex flex-row row-span-2">
                <rive-emoji-face-love id="riveEmojiView" class="block w-full h-full m-1" on:LoadComplete={move |e: MouseEvent| {fetch_likes.dispatch(())}}></rive-emoji-face-love>
                <rive-text id="riveTextView" class="block w-full h-full" on:LoadComplete={move |e: MouseEvent| {count_read_action.dispatch(())}} text={move || {format!("{} views", count_read_action.value().get().as_ref().map(|it| it.to_string()).unwrap_or("...".to_string()))}}></rive-text>
            </div>
        </div>
    }
}
