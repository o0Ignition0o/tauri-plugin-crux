use bincode::Options;
use crux_core::{bridge::ResolveSerialized, App, Core, Effect};
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::thread::JoinHandle;
use tauri::async_runtime::{self, Sender};
use tokio::sync::oneshot;

#[derive(Debug)]
pub(crate) enum Error {
    Send,
    Receive,
}

pub(crate) enum Command<A>
where
    A: App,
{
    ProcessEvent(A::Event),
    HandleResponse(usize, Vec<u8>),
    View,
}

pub struct Crux<A>
where
    A: App,
{
    handle: JoinHandle<()>,
    sender: Sender<(Command<A>, oneshot::Sender<Vec<u8>>)>,
}

impl<A> Crux<A>
where
    A: App + Send + 'static,
    A::Capabilities: Send,
    A::Model: Send,
{
    pub fn new(core: Core<A>) -> Self
    where
        A: App,
    {
        let (sender, mut receiver) =
            async_runtime::channel::<(Command<A>, oneshot::Sender<Vec<u8>>)>(1);
        let mut registry = ResolveRegistry(Default::default());

        let handle = std::thread::spawn(move || {
            while let Some((command, response_sender)) = receiver.blocking_recv() {
                match command {
                    Command::ProcessEvent(event) => {
                        let response = core.process_event(event);
                        let responses = response
                            .into_iter()
                            .map(|effect| {
                                let (eff, resolve) = effect.serialize();
                                let id = registry.0.insert(resolve);
                                Request {
                                    id: EffectId(id.try_into().unwrap()),
                                    effect: eff,
                                }
                            })
                            .collect::<Vec<_>>();

                        let serialized = bincode::serialize(&responses).unwrap();
                        response_sender.send(serialized).unwrap();
                    }
                    Command::HandleResponse(id, response) => {
                        match registry.0.get_mut(id) {
                            Some(resolver) => {
                                // Dirty hack, letsgo
                                let mut deser = bincode::Deserializer::from_slice(
                                    &response,
                                    bincode::DefaultOptions::new()
                                        .with_fixint_encoding()
                                        .allow_trailing_bytes(),
                                );
                                let mut erased_de =
                                    <dyn erased_serde::Deserializer>::erase(&mut deser);
                                resolver.resolve(&mut erased_de).unwrap();

                                // TODO: if resolver::never remove stuff from the registry
                            }
                            None => {
                                panic!("no resolver for effect id {}, this is definitely a bug", id)
                            }
                        };
                        let response = core.process();
                        let responses = response
                            .into_iter()
                            .map(|effect| {
                                let (eff, resolve) = effect.serialize();
                                let id = registry.0.insert(resolve);
                                Request {
                                    id: EffectId(id.try_into().unwrap()),
                                    effect: eff,
                                }
                            })
                            .collect::<Vec<_>>();
                        let serialized = bincode::serialize(&responses).unwrap();
                        response_sender.send(serialized).unwrap();
                    }
                    Command::View => {
                        // Viewmodel is not an Effect so we need to serialize it...
                        let response = core.view();
                        let serialized = bincode::serialize(&response).unwrap();
                        response_sender.send(serialized).unwrap();
                    }
                }
            }
        });
        Self { handle, sender }
    }

    pub fn process_event(&self, event: A::Event) -> Result<Vec<u8>, Error> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .blocking_send((Command::ProcessEvent(event), sender))
            .map_err(|_| Error::Send)?;
        receiver.blocking_recv().map_err(|_| Error::Receive)
    }

    pub fn handle_response(&self, id: usize, response: Vec<u8>) -> Result<Vec<u8>, Error> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .blocking_send((Command::HandleResponse(id, response), sender))
            .map_err(|_| Error::Send)?;
        receiver.blocking_recv().map_err(|_| Error::Receive)
    }

    pub fn view(&self) -> Result<Vec<u8>, Error> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .blocking_send((Command::View, sender))
            .map_err(|_| Error::Send)?;
        receiver.blocking_recv().map_err(|_| Error::Receive)
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

pub struct ResolveRegistry(Slab<ResolveSerialized>);

impl Default for ResolveRegistry {
    fn default() -> Self {
        Self(Slab::with_capacity(1024))
    }
}
