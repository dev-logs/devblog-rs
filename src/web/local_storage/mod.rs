use leptos::window;
use web_sys::Storage;

pub mod user;

pub fn local_storage() -> Storage {
    window().local_storage().expect("Should have local storage").expect("Should have local storage enable")
}
