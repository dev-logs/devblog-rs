use leptos::*;
use leptos::leptos_dom::log;
use wasm_bindgen::JsCast;
use web_sys::FormData;

#[component]
pub fn EditText(
) -> impl IntoView {
    let callback:  Callback<web_sys::SubmitEvent> = (move |submit_event: web_sys::SubmitEvent| {
        // Prevent the default form submission behavior
        submit_event.prevent_default();

        // Get the form element
        let form_element = submit_event.target().unwrap();
        // Access the form values
        let form_data = FormData::new_with_form(&form_element.dyn_into().unwrap()).unwrap();
        let value = form_data.get("comment").as_string().unwrap();
        log!("tiendang-debug {value}");
    }).into();

    view! {
        <form class="mb-6" on:submit=callback>
            <div class="py-2 px-4 mb-4 rounded-lg rounded-t-lg border border-gray-200 bg-gray-800 border-gray-700">
                <label for="comment" class="sr-only">Your comment</label>
                <textarea id="comment" name="comment" rows="6"
                    class="px-0 w-full text-sm border-0 focus:ring-0 focus:outline-none text-white placeholder-gray-400 bg-gray-800"
                    placeholder="Write a comment..." required></textarea>
            </div>
            <button type="submit"
                class="inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-white bg-blue-700 rounded-lg focus:ring-4 focus:ring-primary-200 focus:ring-primary-900 hover:bg-primary-800">
                Submit
            </button>
        </form>
    }
}
