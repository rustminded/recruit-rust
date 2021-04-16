#![cfg(target_arch = "wasm32")]

mod app;
mod contributions;
mod jobs;
mod profile;
mod profile_list_item;
mod profile_not_found;
mod tech_tag;
mod techs;
mod timezone_set;

pub use app::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
    ($s:expr $(,$args:expr)*) => {{
        yew::services::ConsoleService::log(format!($s $(,$args)*).as_str());
    }};
}

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    yew::start_app::<app::App>();
    Ok(())
}
