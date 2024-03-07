use leptos::*;
use web_sys::SubmitEvent;
use crate::core_services::web_di::*;
use crate::services::base::service::*;
use crate::services::subscribe::service::Params;
use crate::web::utils::form_data::FormDataWrapper;
use crate::web::utils::toast::show_welcome_toast;

#[component()]
pub fn MainFooter(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    let subscribe = {
        create_action(move |e: &String| {
            let email = e.clone();
            async move {
                let result = WebInjector::service_injector()
                    .get_subscribe_service()
                    .execute(Params {email: email.clone()})
                    .await;
                show_welcome_toast(email.clone().as_str());
                result
            }

        })
    };

    let on_submit: Callback<SubmitEvent> = Callback::new(move |e: SubmitEvent| {
        e.prevent_default();
        let form_data = FormDataWrapper::from(e);
        let email = form_data.get("email").as_string().unwrap();
        subscribe.dispatch(email.clone());
        form_data.clear();
    });

    view! {
        <footer class=format!("flex flex-col rounded-lg shadow sm:h-1/2 h-fit {class} justify-start items-center bg-gray-950 pb-10")>
            <div class="bg-blue-900 rounded rounded-xl h-1 w-screen mb-24"></div>
            <div class="flex flex-col justify-start sm:items-center items-start sm:mt-40 mt-2">
                <p class="sm:font-main-bold sm:text-4xl font-main text-2xl">Thank you for your visiting</p>
                <div class="flex flex-col justify-center sm:mt-12 mt-2 sm:mb-20 mb-2">
                    <h2 class="sm:text-xl text-lg font-main mb-4 text-white">Subscribe to our Newsletter</h2>
                    <form class="sm:w-96 w-80" on:submit=on_submit>
                        <div class="flex rounded-full mb-4 sm:h-12 h-8">
                            <input class="bg-white rounded rounded-md w-full font-main text-lg text-gray-950 py-1 h-full px-2 leading-tight focus:outline-none" type="email" placeholder="Enter your email" aria-label="Email" name="email"/>
                            <button
                                type="submit"
                                class="flex-shrink-0 font-main bg-blue-500 h-full hover:bg-blue-700 border-blue-500 hover:border-blue-700 text-sm border-4 text-white py-1 px-2 rounded ml-4" type="button">
                                Subscribe
                            </button>
                        </div>
                    </form>
                    <p class="font-main text-sm text-red800">{move || {error_message(subscribe.value().get())}}</p>
                </div>
            </div>
            <div class="grid grid-cols-10 gap-8 max-w-screen-3xl sm:mt-24 mt-6">
                <div class="col-span-4 flex flex-col justify-between items-center p-4">
                    <div class="flex flex-col justify-start items-start">
                        <div class="flex flex-row mb-4">
                            <img class="h-10 w-10 rounded rounded-md" src="/assets/images/ic_devlog.png" alt="logo"/>
                            <p class="font-main-bold text-xl ml-6 text-blue-500">Devlog</p>
                        </div>
                        <span class="font-main sm:text-md text-sm text-gray-300 max-w-72">
                            Providing high-quality content to engage our community and attract quality customers.
                            Continuously learning to establish a strong foundation and optimize costs.
                            Get in touch with us here
                        </span>
                    </div>
                </div>
                <div class="col-span-3 flex flex-col p-4 justify-start items-start">
                    <div class="flex flex-row justify-center items-center mb-4">
                        <p class="font-main-bold text-lg text-blue-500">Social media</p>
                    </div>
                    <ul class="list">
                        <li class="font-main sm:text-md text-sm text-gray-300">Instagram</li>
                        <li class="font-main sm:text-md text-sm text-gray-600">Sharing interesting tips on programing</li>
                        <li class="font-main sm:text-md text-sm text-gray-300">Youtube</li>
                        <li class="font-main sm:text-md text-sm text-gray-600">Live streaming and tutorial on about coding</li>
                    </ul>
                </div>
                <div class="col-span-3 flex flex-col p-4 justify-start items-start">
                    <div class="flex flex-row justify-center items-center mb-4">
                        <p class="font-main-bold text-lg text-blue-500">Products</p>
                    </div>
                    <ul class="list font-main">
                        <li class="font-main sm:text-md text-sm text-gray-300">Modboxify</li>
                        <li class="font-main sm:text-md text-sm text-gray-500">Assistant to advance your brain</li>
                    </ul>
                </div>
            </div>
            <div class="max-w-screen-3xl justify-start items-start py-4 px-24">
                <p class="font-main sm:text-lg text-sm text-gray-600"> {"©"} 2024 Devlog{"™"}. All Rights Reserved.</p>
            </div>
        </footer>
    }
}