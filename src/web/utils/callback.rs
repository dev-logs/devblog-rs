use leptos::Callback;

pub fn noop_callback<In: 'static>() -> Callback<In> {
    Callback::new(move |_: In| {})
}