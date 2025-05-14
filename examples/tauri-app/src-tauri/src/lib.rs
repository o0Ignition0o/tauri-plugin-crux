mod crux_example;
use shared::{App, Effect, Core};
use tauri_plugin_crux::reexports::crux_core;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let crux_core = Core::<App>::new();

    let effect_handler = |core: &Core<App>, effect: shared::Effect| {
        let effects_to_forward = match effect {
            shared::Effect::Http(mut request) => {
                // TODO: native call?
                shared::Effect::Http(request)
            },
            rest => rest
        };

        dbg!(&effects_to_forward);
        vec![effects_to_forward]
    
    };

    let devtools = tauri_plugin_devtools::init();
    tauri::Builder::default()
        .plugin(devtools)
        .plugin(tauri_plugin_crux::init(crux_core, effect_handler))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
