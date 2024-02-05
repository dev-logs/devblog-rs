use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{FormData, SubmitEvent};

pub struct FormDataWrapper {
    form_data: FormData
}

impl From<SubmitEvent> for FormDataWrapper {
    fn from(submit_event: SubmitEvent) -> Self {
        let form_element = submit_event.target().unwrap();
        let form_data = FormData::new_with_form(&form_element.dyn_into().unwrap()).unwrap();

        Self { form_data }
    }
}

impl Deref for FormDataWrapper {
    type Target = FormData;

    fn deref(&self) -> &Self::Target {
        &self.form_data
    }
}
