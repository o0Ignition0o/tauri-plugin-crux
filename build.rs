const COMMANDS: &[&str] = &["process_event", "view", "handle_response"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
