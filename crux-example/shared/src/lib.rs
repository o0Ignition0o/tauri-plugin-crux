pub use app::*;
pub use crux_core::{Core, Request, bridge::Bridge};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;
pub mod capabilities;
pub use capabilities::sse;

uniffi::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Bridge<App> = Bridge::new(Core::new());
}

#[wasm_bindgen]
pub fn process_event(data: &[u8]) -> Vec<u8> {
    match CORE.process_event(data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

#[wasm_bindgen]
pub fn handle_response(id: u32, data: &[u8]) -> Vec<u8> {
    match CORE.handle_response(id, data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    match CORE.view() {
        Ok(view) => view,
        Err(e) => panic!("{e}"),
    }
}
