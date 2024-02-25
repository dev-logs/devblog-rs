use std::ops::Deref;
use leptos::logging::log;
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement, HtmlTextAreaElement, SubmitEvent};
use web_sys::js_sys::eval;

pub struct FormDataWrapper {
    form_data: FormData,
    element: HtmlFormElement
}

impl From<SubmitEvent> for FormDataWrapper {
    fn from(submit_event: SubmitEvent) -> Self {
        let target = submit_event.target().unwrap();

        let form_data = FormData::new_with_form(&target.clone().dyn_into().unwrap()).unwrap();

        let form = target.dyn_into::<HtmlFormElement>().unwrap();
        Self { form_data, element: form }
    }
}

impl Deref for FormDataWrapper {
    type Target = FormData;

    fn deref(&self) -> &Self::Target {
        &self.form_data
    }
}

impl FormDataWrapper {
    pub fn clear(&self) {
        let inputs = self.element.query_selector_all("input").unwrap();
        for i in 0..inputs.length() {
            {
                let input = inputs.get(i).unwrap().dyn_into::<HtmlInputElement>().unwrap();
                input.set_value("");
                input.set_text_content(Some(""));
                log!("Input value: {:?}", input.value());
            }
        }

        let textareas = self.element.query_selector_all("textarea").unwrap();
        for i in 0..textareas.length() {
            let textarea = textareas.get(i).unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
            let textarea_value = textarea.value();
            textarea.set_value("");
            textarea.set_text_content(Some(""));
            log!("Textarea value: {:?}", textarea_value);
        }
    }
}