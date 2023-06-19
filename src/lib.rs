use cfg_if::cfg_if;

pub mod app;
pub mod db;
pub mod rate_repository;
pub mod rates;


cfg_if! {
if #[cfg(feature = "hydrate")] {

    use crate::app::*;
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {

      console_error_panic_hook::set_once();


      leptos::mount_to_body(move |cx| {
          view! { cx, <App/> }
      });
    }
}
}
