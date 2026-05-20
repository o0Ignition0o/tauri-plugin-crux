use base64::{prelude::BASE64_STANDARD, Engine};
use crux::Crux;
use crux_core::{bridge::Bridge, App, EffectFFI};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::{
    ipc::InvokeBody,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub mod reexports {
    pub use crux_core;
}

pub use models::*;

mod crux;
mod error;
mod models;

pub use error::{Error, Result};

#[derive(Deserialize)]
pub struct TempResponse {
    id: u32,
    response: Vec<u8>,
}

/// Initializes the plugin.
pub fn init<R: Runtime, A>(crux_app: Bridge<A>) -> TauriPlugin<R>
where
    A: App + Send + Sync + 'static,
    A::Model: Send + Sync,
    A::Effect: EffectFFI,
    A::ViewModel: Serialize,
    A::Event: DeserializeOwned,
{
    let crux = Crux::new(crux_app);

    Builder::new("crux")
        // It is recommended you use the tauri::generate_handler to generate the input to this method, as the input type is not considered stable yet.
        // Unfortunately the macro doesn't play nice with generics, so we'll have to handle it manually for now
        .invoke_handler(move |handler| {
            match handler.message.command() {
                "process_event" => {
                    let event: Vec<u8> = match handler.message.payload() {
                        InvokeBody::Json(payload) => {
                            // Possible values of an IPC payload.
                            //
                            // ### Android
                            // On Android, [InvokeBody::Raw] is not supported. The enum will always contain [InvokeBody::Json].
                            // When targeting Android Devices, consider passing raw bytes as a base64 [[std::string::String]], which is still more efficient than passing them as a number array in [InvokeBody::Json]
                            let b64: String = serde_json::from_value(payload.clone()).unwrap();
                            BASE64_STANDARD.decode(b64).unwrap()
                        }
                        InvokeBody::Raw(bytes) => bytes.clone(), // TODO: no need to clone that fr
                    };

                    handler.resolver.resolve(crux.process_event(event).unwrap());
                    true
                }
                "handle_response" => {
                    let (id, response): (u32, Vec<u8>) = match handler.message.payload() {
                        // TODO: we whouldnt hit that path tbh
                        InvokeBody::Json(payload) => {
                            let response: TempResponse =
                                serde_json::from_value(payload.clone()).unwrap();
                            (response.id, response.response)
                        }
                        InvokeBody::Raw(bytes) => bincode::deserialize(bytes).unwrap(),
                    };
                    handler
                        .resolver
                        .resolve(crux.handle_response(id, response).unwrap());
                    true
                }
                "view" => {
                    handler.resolver.resolve(crux.view().unwrap());
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
