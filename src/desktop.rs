use std::sync::Arc;

use crux_core::{App, Core};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

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
pub struct Crux<R: Runtime, A>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    app: AppHandle<R>,
    core: Arc<Core<A>>,
}

impl<R: Runtime, A> Crux<R, A>
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

impl<R: Runtime, A> Crux<R, A>
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
