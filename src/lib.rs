#![feature(iter_collect_into)]
#![feature(async_closure)]
pub mod app;
pub mod web;
pub mod entities;
pub mod services;
pub mod core_services;
pub mod api;
pub mod macros;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      mount_to(leptos::document().head().take().unwrap().into(), move || view! {
        <script type="module" src="/assets/js/react/index.js"/>
      });
      mount_to_body(App);
    }
}
}
