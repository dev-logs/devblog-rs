use leptos::*;
use web_sys::js_sys::eval;
use web_sys::{MouseEvent, SubmitEvent};
use crate::entities::user::User;
use crate::web::components::icons::close::Close;
use crate::web::components::icons::github::Github;
use crate::web::components::icons::google::Google;
use crate::web::local_storage::user::{UserStorage};
use crate::web::utils::form_data::FormDataWrapper;

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

    create_effect(move |_| {
        let animation = "{y: '70vh'}, {y: '0', duration: 2.5, ease: 'elastic.inOut(0.5,0.4)'}";

        eval(format!(r#"
            gsap.fromTo('.modal', {animation})
        "#).as_str()).unwrap();
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
    on_dismiss_click: Callback<MouseEvent>,
    #[prop(default = None)]
    on_submit: Option<Callback<UserStorage>>
) -> impl IntoView {
    let on_submit_clicked: Callback<SubmitEvent> = Callback::new(move |submit_event: SubmitEvent| {
        submit_event.prevent_default();
        let form_data = FormDataWrapper::from(submit_event);
        let display_name = form_data.get("displayname").as_string().unwrap();
        let mut user_storage = UserStorage::new();
        user_storage.update(User::new(None, None, display_name.as_str()));
    });

    view! {
        <div class="fixed top-0 left-0 w-full h-full bg-gray-800 bg-opacity-50 backdrop-filter backdrop-blur-lg z-10 justify-center items-center">
            <div class="flex flex-col justify-center items-center h-full">
                <div class="modal flex flex-col bg-indigo-100 rounded-xl">
                    <header class="relative">
                        <button on:click=on_dismiss_click class="absolute right-0 pt-3 pr-3"><Close/></button>
                        <img class="w-20 mx-auto mb-5" src="/assets/images/ic_devlog_transparent.png" />
                    </header>
                    <form on:submit=on_submit_clicked class="pt-2 pb-5 px-10">
                        <div>
                            <label class="block mb-2 text-indigo-500 text-md" for="displayname">Display name</label>
                            <input class="w-full p-2 mb-4 text-indigo-700 border-b-2 border-indigo-500 outline-none focus:bg-gray-300" type="text" name="displayname"/>
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
            <script src="https://cdnjs.cloudflare.com/ajax/libs/gsap/3.12.5/gsap.min.js">
            </script>
        </div>
    }
}
