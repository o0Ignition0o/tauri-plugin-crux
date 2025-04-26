use std::sync::Arc;

use crux_core::{App, Core};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned, A>(
    core: Core<A>,
    app: AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Crux<R, C, A>>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    Ok(Crux {
        app,
        _api,
        core: Arc::new(core),
    })
}

/// Access to the crux APIs.
pub struct Crux<R: Runtime, C: DeserializeOwned, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    app: AppHandle<R>,
    _api: PluginApi<R, C>,
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
