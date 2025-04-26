use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_crux);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    crux_core: Core<A>,
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Crux<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.crux", "ExamplePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_crux)?;
    Ok(Crux::new(crux_core, handle))
}
pub fn init<R: Runtime, C: DeserializeOwned, A>(
    crux_core: Core<A>,
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Crux<R, A>>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    Ok(Crux::new(app.clone(), crux_core))
}

/// Access to the crux APIs.
pub struct Crux<R: Runtime, C: DeserializeOwned, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    api: PluginApi<R, C>,
    core: Arc<Core<A>>,
}

impl<R: Runtime, C: DeserializeOwned, A> Crux<R: Runtime, C: DeserializeOwned, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    pub fn new(app: AppHandle<R>, core: Core<A>) -> Self {
        Self {
            app,
            core: Arc::new(core),
        }
    }
}

impl<R: Runtime, C: DeserializeOwned, A> Crux<R: Runtime, C: DeserializeOwned, A>
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
