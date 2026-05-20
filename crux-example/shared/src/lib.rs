pub use app::*;
pub use crux_core::{
    bridge::{Bridge, EffectId},
    Core, Request,
};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;
pub mod capabilities;
pub use capabilities::sse;

// #[cfg(not(target_arch = "wasm32"))]
// uniffi::include_scaffolding!("shared");

lazy_static! {
    static ref CORE: Bridge<App> = Bridge::new(Core::new());
}

#[wasm_bindgen]
pub fn process_event(data: &[u8]) -> Vec<u8> {
    let mut out_buf = Vec::new();
    CORE.update(data, &mut out_buf).unwrap();
    out_buf
}

#[wasm_bindgen]
pub fn handle_response(id: u32, data: &[u8]) -> Vec<u8> {
    println!("handle response called");
    let mut out_buf = Vec::new();
    CORE.resolve(EffectId(id), data, &mut out_buf).unwrap();
    out_buf
}

#[wasm_bindgen]
pub fn view() -> Vec<u8> {
    let mut out_buf = Vec::new();
    CORE.view(&mut out_buf).unwrap();
    out_buf
}
