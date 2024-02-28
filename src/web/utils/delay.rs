use std::time::Duration;
use leptos::window;
use wasm_bindgen::closure::Closure;
use tokio::sync::oneshot;
use wasm_bindgen::JsCast;

pub async fn delay(duration: Duration) {
    let (tx, rx) = oneshot::channel::<()>();

    let callback = Closure::once(move || {
        tx.send(());
    });

    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), duration.as_millis() as i32)
        .expect("Failed to set timeout");

    rx.await.expect("Failed to receive timeout signal");
}