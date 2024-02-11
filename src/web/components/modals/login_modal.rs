use leptos::*;
use web_sys::MouseEvent;
use crate::web::components::icons::close::Close;
use crate::web::components::icons::github::Github;
use crate::web::components::icons::google::Google;

#[component]
pub fn login_modal(
    #[prop(default = false)]
    is_show: bool
) -> impl IntoView {
    let (show, set_show) = create_signal(is_show);

    let empty_view = view!{<div></div>};
    let on_dismiss_click: Callback<MouseEvent> = Callback::new(move |_| {
        set_show(!show.get());
    });

    view ! {
        {move || {
            if show.get() {
                view!{<div><LoginForm on_dismiss_click=on_dismiss_click/></div>}
            }
            else {
                view!{<div/>}
            }
        }}
    }
}

#[component]
fn LoginForm(
    #[prop()]
    on_dismiss_click: Callback<MouseEvent>
) -> impl IntoView {
    view! {
        <div class="fixed top-0 left-0 w-full h-full bg-gray-800 bg-opacity-50 backdrop-filter backdrop-blur-lg z-10 justify-center items-center">
            <div class="flex flex-col justify-center items-center h-full">
                <div class="flex flex-col bg-indigo-100 rounded-xl">
                <header class="relative">
                    <button on:click=on_dismiss_click class="absolute right-0 pt-3 pr-3"><Close/></button>
                    <img class="w-20 mx-auto mb-5" src="/assets/images/ic_devlog_transparent.png" />
                </header>
                <form class="pt-2 pb-5 px-10">
                    <div>
                        <label class="block mb-2 text-indigo-500 text-md" for="username">Display name</label>
                        <input class="w-full p-2 mb-4 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="username"/>
                    </div>
                    <div>
                        <button class="w-full text-indigo-700 hover:text-indigo-300 text-white text-md font-bold px-4" type="submit">
                            Continue
                        </button>
                    </div>
                </form>
                <p class="font-bold text-center mt-10">Or login with</p>
                <footer class="flex flex-row justify-center space-x-9 items-center">
                    <button class="bg-white shadow-gray-300 shadow-lg rounded-xl hover:bg-gray-300 text-white font-bold py-2 px-4 mb-6 rounded" type="submit">
                        <Github/>
                    </button>
                    <button class="bg-white shadow-gray-300 shadow-lg rounded-xl hover:bg-gray-300 text-white font-bold py-2 px-4 mb-6 rounded" type="submit">
                        <Google/>
                    </button>
                </footer>
            </div>
            </div>
        </div>
    }
}
