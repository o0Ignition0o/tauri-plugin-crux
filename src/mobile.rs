use std::sync::Arc;

use crux_core::{App, Core};
use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_crux);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned, A>(
    crux_core: Core<A>,
    _app: AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Crux<R, C, A>>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.crux", "Crux")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_crux)?;
    Ok(Crux {
        _app,
        api,
        core: Arc::new(crux_core),
    })
}

/// Access to the crux APIs.
pub struct Crux<R: Runtime, C: DeserializeOwned, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    api: PluginApi<R, C>,
    _app: AppHandle<R>,
    core: Arc<Core<A>>,
}

impl<R: Runtime, C: DeserializeOwned, A> Crux<R, C, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    pub fn process_event(&self, event: A::Event) {
        println!("process event called");
    }

    pub fn handle_response(&self, response: A::Effect) {
        println!("handle response called");
    }

    pub fn view() {
        println!("view called");
    }
}
