use leptos::*;

#[component()]
pub fn MainFooter(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    view! {
        <footer class=format!("flex flex-row rounded-lg shadow pt-12 h-screen {class} p-4 pt-12 justify-center")>
            <div class="grid grid-cols-10 gap-8 max-w-screen-2xl">
                <div class="col-span-4 flex flex-col justify-between items-center p-4">
                    <div class="flex flex-col justify-start items-start p-4">
                        <div class="flex flex-row justify-center items-center h-40">
                            <img class="rounded rounded-xl" src="/assets/images/ic_devlog.png" alt="logo"/>
                            <p class="font-main-bold text-4xl ml-6 text-blue-500">Devlog</p>
                        </div>
                        <span class="font-main text-lg text-gray-400 max-w-72">
                            Providing high-quality content to engage our community and attract quality customers.
                            Continuously learning to establish a strong foundation and optimize costs.
                            Get in touch with us here
                        </span>
                    </div>
                </div>
                <div class="col-span-3 flex flex-col p-4 justify-start items-start">
                    <div class="flex flex-row justify-center items-center h-40">
                        <p class="font-main-bold font-bold text-4xl text-blue-500">Social media</p>
                    </div>
                    <ul class="list">
                        <li class="text-lg font-main text-lg text-gray-300">Instagram</li>
                        <li class="text-lg font-main text-lg text-gray-600">Sharing interesting tips on programing</li>
                        <li class="text-lg font-main text-lg text-gray-300">Youtube</li>
                        <li class="text-lg font-main text-lg text-gray-600">Live streaming and tutorial on about coding</li>
                    </ul>
                </div>
                <div class="col-span-3 flex flex-col p-4 justify-start items-start">
                    <div class="flex flex-row justify-center items-center h-40">
                        <p class="font-main-bold font-bold text-4xl text-blue-500">Products</p>
                    </div>
                    <ul class="list text-lg font-main">
                        <li class="text-lg font-main text-gray-300">Modboxify</li>
                        <li class="text-lg font-main text-gray-500">Assistant to advance your brain</li>
                    </ul>
                </div>
            </div>
        </footer>
    }
}