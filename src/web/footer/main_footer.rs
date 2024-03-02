use leptos::*;

#[component()]
pub fn MainFooter(
    #[prop(default = "")]
    class: &'static str
) -> impl IntoView {
    view! {
        <footer class=format!("flex flex-col rounded-lg shadow h-1/2 {class} justify-start items-center bg-gray-950")>
            <div class="bg-blue-800 rounded rounded-xl h-1 w-screen mb-24"></div>
            <div class="flex flex-col justify-start items-center mt-2">
                <p class="font-main-bold text-4xl">Thanks for your visiting</p>
                <div class="flex flex-col justify-center mt-12">
                    <h2 class="text-xl font-main mb-4 text-white">Subscribe to our Newsletter</h2>
                    <form class="w-full max-w-sm">
                        <div class="flex rounded-full mb-4 h-12">
                            <input class="bg-white rounded rounded-md w-full font-main text-lg text-gray-950 py-1 h-full px-2 leading-tight focus:outline-none" type="email" placeholder="Enter your email" aria-label="Email"/>
                            <button class="flex-shrink-0 font-main bg-blue-500 h-full hover:bg-blue-700 border-blue-500 hover:border-blue-700 text-sm border-4 text-white py-1 px-2 rounded ml-4" type="button">
                                Subscribe
                            </button>
                        </div>
                    </form>
                </div>
            </div>
            <div class="grid grid-cols-10 gap-8 max-w-screen-2xl mt-24">
                <div class="col-span-4 flex flex-col justify-between items-center p-4">
                    <div class="flex flex-col justify-start items-start">
                        <div class="flex flex-row mb-4">
                            <img class="h-10 w-10 rounded rounded-md" src="/assets/images/ic_devlog.png" alt="logo"/>
                            <p class="font-main-bold text-xl ml-6 text-blue-500">Devlog</p>
                        </div>
                        <span class="font-main text-md text-gray-300 max-w-72">
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
                        <li class="font-main text-md text-gray-300">Instagram</li>
                        <li class="font-main text-md text-gray-600">Sharing interesting tips on programing</li>
                        <li class="font-main text-md text-gray-300">Youtube</li>
                        <li class="font-main text-md text-gray-600">Live streaming and tutorial on about coding</li>
                    </ul>
                </div>
                <div class="col-span-3 flex flex-col p-4 justify-start items-start">
                    <div class="flex flex-row justify-center items-center mb-4">
                        <p class="font-main-bold text-lg text-blue-500">Products</p>
                    </div>
                    <ul class="list font-main">
                        <li class="font-main text-md text-gray-300">Modboxify</li>
                        <li class="font-main text-md text-gray-500">Assistant to advance your brain</li>
                    </ul>
                </div>
            </div>
            <div class="max-w-screen-2xl justify-start items-start py-4 px-24">
                <p class="font-main font-lg text-gray-600"> {"©"} 2024 Devlog{"™"}. All Rights Reserved.</p>
            </div>
        </footer>
    }
}