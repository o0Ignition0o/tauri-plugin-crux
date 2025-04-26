use std::sync::{Arc, Mutex};

use base64::prelude::*;
use bincode::Options;
use crux_core::{bridge::ResolveSerialized, App, Core, Effect};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use slab::Slab;
use tauri::{
    ipc::InvokeBody,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Crux;
#[cfg(mobile)]
use mobile::Crux;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the crux APIs.
pub trait CruxExt<R: Runtime, C: DeserializeOwned, A: App>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    fn crux(&self) -> &Crux<R, C, A>;
}

impl<R: Runtime, C: DeserializeOwned, T: Manager<R>, A> crate::CruxExt<R, C, A> for T
where
    C: Send + Sync + 'static,
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
{
    fn crux(&self) -> &Crux<R, C, A> {
        self.state::<Crux<R, C, A>>().inner()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EffectId(pub u32);

#[derive(Debug, Serialize)]
pub struct Request<Eff>
where
    Eff: Serialize,
{
    pub id: EffectId,
    pub effect: Eff,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: usize,
    pub response: Vec<u8>,
}

pub struct ResolveRegistry(Mutex<Slab<ResolveSerialized>>);

impl Default for ResolveRegistry {
    fn default() -> Self {
        Self(Mutex::new(Slab::with_capacity(1024)))
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime, A>(crux_core: Core<A>) -> TauriPlugin<R>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Capabilities: Send + Sync,
    A::Event: DeserializeOwned,
{
    let registry = ResolveRegistry(Default::default());
    let crux = Arc::new(crux_core);
    Builder::new("crux")
        // It is recommended you use the tauri::generate_handler to generate the input to this method, as the input type is not considered stable yet.
        // Unfortunately the macro doesn't play nice with generics, so we'll have to handle it manually for now
        .invoke_handler(move |handler| {
            match handler.message.command() {
                "process_event" => {
                    let event: A::Event = match handler.message.payload() {
                        InvokeBody::Json(payload) => {
                            // Possible values of an IPC payload.
                            //
                            // ### Android
                            // On Android, [InvokeBody::Raw] is not supported. The enum will always contain [InvokeBody::Json].
                            // When targeting Android Devices, consider passing raw bytes as a base64 [[std::string::String]], which is still more efficient than passing them as a number array in [InvokeBody::Json]
                            let b64: String = serde_json::from_value(payload.clone()).unwrap();
                            let raw = BASE64_STANDARD.decode(b64).unwrap();
                            bincode::deserialize(&raw)
                        }
                        InvokeBody::Raw(bytes) => bincode::deserialize(bytes),
                    }
                    .unwrap();
                    let responses = crux
                        .process_event(event)
                        .into_iter()
                        .map(|effect| {
                            let (eff, resolve) = effect.serialize();
                            let id = registry.0.lock().unwrap().insert(resolve);
                            Request {
                                id: EffectId(id.try_into().unwrap()),
                                effect: eff,
                            }
                        })
                        .collect::<Vec<_>>();

                    let serialized = bincode::serialize(&responses).unwrap();
                    handler.resolver.resolve(serialized);
                    true
                }
                "handle_response" => {
                    let (id, response): (usize, Vec<u8>) = match handler.message.payload() {
                        InvokeBody::Json(payload) => {
                            let response: Response =
                                serde_json::from_value(payload.clone()).unwrap();
                            (response.id, response.response)
                        }
                        InvokeBody::Raw(bytes) => bincode::deserialize(bytes).unwrap(),
                    };

                    // TODO: we might be holding the lock a bit too long here
                    match registry.0.lock().unwrap().get_mut(id) {
                        Some(resolver) => {
                            // Dirty hack, letsgo
                            let mut deser = bincode::Deserializer::from_slice(
                                &response,
                                bincode::DefaultOptions::new()
                                    .with_fixint_encoding()
                                    .allow_trailing_bytes(),
                            );
                            let mut erased_de = <dyn erased_serde::Deserializer>::erase(&mut deser);
                            resolver.resolve(&mut erased_de).unwrap();
                        }
                        None => {
                            panic!("no resolver for effect id {}, this is definitely a bug", id)
                        }
                    };
                    true
                }
                "view" => {
                    let serialized = bincode::serialize(&crux.view()).unwrap();
                    handler.resolver.resolve(serialized);
                    true
                }
                _ => {
                    println!("unknown event {}", handler.message.command());
                    false
                }
            }
        })
        .build()
}
