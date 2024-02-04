#![feature(iter_collect_into)]

pub mod app;
pub mod web;
pub mod entities;
pub mod services;
pub mod core_services;
pub mod api;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      mount_to_body(App);
    }
}
}
