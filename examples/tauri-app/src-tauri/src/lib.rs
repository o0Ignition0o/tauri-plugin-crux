mod crux_example;
use shared::{Core, Counter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let crux_core = Core::<Counter>::new();

    let devtools = tauri_plugin_devtools::init();
    tauri::Builder::default()
        .plugin(devtools)
        .plugin(tauri_plugin_crux::init(crux_core))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
