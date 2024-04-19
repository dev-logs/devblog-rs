use leptos::*;
use web_sys::js_sys::eval;
use web_sys::{MouseEvent, SubmitEvent};
use crate::core_services::web_di::{WebInjector, WebServicesInjector};
use crate::services::create_guess_user::service::*;
use crate::services::base::service::*;
use crate::web::components::icons::close::Close;
use crate::web::components::icons::github::Github;
use crate::web::components::icons::google::Google;
use crate::web::utils::form_data::FormDataWrapper;
use crate::web::utils::toast::show_welcome_toast;
use crate::include_js;

#[component]
pub fn login_modal(
    #[prop(default = false)]
    is_show: bool
) -> impl IntoView {
    let (show, set_show) = create_signal(is_show);

    let on_dismiss_click: Callback<MouseEvent> = Callback::new(move |_| {
        set_show(!show.get());
    });

    let create_user_action = {
        let set_show = set_show.clone();
        create_action(move |display_name: &String| {
            let display_name = display_name.clone();
            async move {
                let result = WebInjector::service_injector().get_create_guest_user_service().execute(Params {
                    display_name: display_name.clone()
                }).await;

                if result.is_ok() {
                    show_welcome_toast(display_name.as_str());
                    set_show(false);
                }

                result
            }
        })
    };

    let on_submit_clicked: Callback<SubmitEvent> = Callback::new(move |submit_event: SubmitEvent| {
        submit_event.prevent_default();
        let form_data = FormDataWrapper::from(submit_event);
        let display_name = form_data.get("displayname").as_string().unwrap();
        create_user_action.dispatch(display_name);
    });

    create_effect(move |_| {
        eval(include_js! {
            gsap.fromTo(".modal", {y: "70vh"}, {y: "0", duration: 2.5, ease: "elastic.inOut(0.5,0.4)"})
        }.as_str()).unwrap();
    });

    view ! {
        {move || {
            if show.get() {
                view!{<div><LoginForm error_message={error_message(create_user_action.value().get())} on_dismiss_click=on_dismiss_click on_submit=on_submit_clicked/></div>}
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
    #[prop()]
    on_submit: Callback<SubmitEvent>,
    #[prop(default = None)]
    error_message: Option<String>
) -> impl IntoView {
    view! {
        <div class="fixed top-0 left-0 w-full h-full bg-gray-800 bg-opacity-50 backdrop-filter backdrop-blur-lg z-10 justify-center items-center">
            <div class="flex flex-col justify-center items-center h-full">
                <div class="modal flex flex-col bg-indigo-100 rounded-xl">
                    <header class="relative">
                        <button on:click=on_dismiss_click class="absolute right-0 pt-3 pr-3"><Close/></button>
                        <img class="w-20 mx-auto mb-5" src="/assets/images/ic_devlog_transparent.png" />
                    </header>
                    <form on:submit=on_submit class="pt-2 pb-5 px-10">
                        <div>
                            <label class="block mb-2 text-indigo-500 text-md font-main.jsx" for="displayname">Display name</label>
                            <input class="w-full p-2 mb-4 text-indigo-700 border-b-2 font-main.jsx border-indigo-500 outline-none focus:bg-gray-300" type="text" name="displayname" required/>
                            <p class="text-red-800 font-main.jsx">{error_message.unwrap_or("".to_string())}</p>
                        </div>
                        <div>
                            <button class="w-full text-indigo-700 font-main.jsx hover:text-indigo-300 text-md font-main.jsx-bold px-4" type="submit">
                                Continue
                            </button>
                        </div>
                    </form>
                    <p class="font-main.jsx-bold text-center mt-10">Or login with</p>
                    <footer class="flex flex-row justify-center space-x-9 items-center">
                        <button class="bg-white shadow-gray-300 shadow-lg rounded-xl hover:bg-gray-300 text-gray-200 font-main.jsx-bold py-2 px-4 mb-6 rounded" type="submit">
                            <Github/>
                        </button>
                        <button class="bg-white shadow-gray-300 shadow-lg rounded-xl hover:bg-gray-300 text-gray-200 font-main.jsx-bold py-2 px-4 mb-6 rounded" type="submit">
                            <Google/>
                        </button>
                    </footer>
                </div>
            </div>
        </div>
    }
}
