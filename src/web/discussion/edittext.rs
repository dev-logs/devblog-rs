use leptos::*;
use crate::web::utils::form_data::FormDataWrapper;

#[derive(Debug, Clone)]
pub struct DiscussionSubmitEvent {
    pub(crate) content: String
}

#[component]
pub fn EditText(
    #[prop()]
    callback: Callback<DiscussionSubmitEvent>
) -> impl IntoView {
    let (event, set_event) = create_signal(DiscussionSubmitEvent {content: String::new()});

    let on_submit: Callback<web_sys::SubmitEvent> = Callback::new(move |submit_event: web_sys::SubmitEvent| {
        submit_event.prevent_default();

        let form_data = FormDataWrapper::from(submit_event);
        (callback).call(DiscussionSubmitEvent {
           content: form_data.get("comment").as_string().unwrap()
        })
    });

    view! {
        <form class="mb-6" on:submit=on_submit>
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
