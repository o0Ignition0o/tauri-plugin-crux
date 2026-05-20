use crux_core::{
    bridge::{Bridge, EffectId},
    App, EffectFFI,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{marker::PhantomData, thread::JoinHandle};
use tauri::async_runtime::{self, Sender};
use tokio::sync::oneshot;

#[derive(Debug)]
pub(crate) enum Error {
    Send,
    Receive,
}

pub(crate) enum Command {
    ProcessEvent(Vec<u8>),
    HandleResponse(EffectId, Vec<u8>),
    View,
}

pub struct Crux<A>
where
    A: App,
{
    _handle: JoinHandle<()>,
    sender: Sender<(Command, oneshot::Sender<Vec<u8>>)>,
    app: PhantomData<A>,
}

impl<'a, A> Crux<A>
where
    A: App + Send + 'static,
    A::Event: DeserializeOwned,
    A::Effect: EffectFFI,
    A::Model: Send,
    A::ViewModel: Serialize,
{
    pub fn new(core: Bridge<A>) -> Self
    where
        A: App,
    {
        let (sender, mut receiver) =
            async_runtime::channel::<(Command, oneshot::Sender<Vec<u8>>)>(1);

        let _handle = std::thread::spawn(move || {
            while let Some((command, response_sender)) = receiver.blocking_recv() {
                let mut response_output = Vec::new();

                match command {
                    Command::ProcessEvent(event) => {
                        core.update(&event, &mut response_output).unwrap();
                        response_sender.send(response_output).unwrap();
                    }
                    Command::HandleResponse(id, response) => {
                        tracing::error!(id = ?id, "handling response");
                        core.resolve(id, &response, &mut response_output).unwrap();
                        response_sender.send(response_output).unwrap();
                    }
                    Command::View => {
                        core.view(&mut response_output).unwrap();
                        response_sender.send(response_output).unwrap();
                    }
                }
            }
        });
        Self {
            _handle,
            sender,
            app: Default::default(),
        }
    }

    pub fn process_event(&self, event: Vec<u8>) -> Result<Vec<u8>, Error> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .blocking_send((Command::ProcessEvent(event), sender))
            .map_err(|_| Error::Send)?;
        receiver.blocking_recv().map_err(|_| Error::Receive)
    }

    pub fn handle_response(&self, id: u32, response: Vec<u8>) -> Result<Vec<u8>, Error> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .blocking_send((Command::HandleResponse(EffectId(id), response), sender))
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
