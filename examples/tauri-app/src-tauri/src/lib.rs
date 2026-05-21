mod crux_example;
use crux_core::bridge::Bridge;
use shared::{App, Core};
use tauri_plugin_crux::reexports::crux_core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let crux_core = Core::<App>::new();
    let bridge = Bridge::new(crux_core);

    let devtools = tauri_plugin_devtools::init();
    tauri::Builder::default()
        .plugin(devtools)
        .plugin(tauri_plugin_crux::init(bridge))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
