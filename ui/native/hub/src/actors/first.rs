use crate::signals::{ServerMessage, SmallText};
use async_trait::async_trait;
use client::say_hello;
use messages::prelude::{Actor, Address, Context, Notifiable};
use rinf::{DartSignal, RustSignal, debug_print};
#[cfg(target_arch = "wasm32")]
use send_wrapper::SendWrapper;
use tokio::task::JoinSet;

// Uncomment below to target the web.
use tokio_with_wasm::alias as tokio;

/// The first actor.
pub struct FirstActor {
    /// Owned tasks that are canceled when the actor is dropped.
    _owned_tasks: JoinSet<()>,
}

// Implementing the `Actor` trait for `CountingActor`.
// This defines `FirstActor` as an actor in the async system.
impl Actor for FirstActor {}

impl FirstActor {
    /// Creates the actor and initializes its fields.
    pub fn new(self_addr: Address<Self>) -> Self {
        let mut _owned_tasks = JoinSet::new();
        _owned_tasks.spawn(Self::listen_to_dart(self_addr));
        FirstActor { _owned_tasks }
    }
}

// Implementing the `Notifiable` trait
// allows an actor's loop to listen for a specific message type.
#[async_trait]
impl Notifiable<SmallText> for FirstActor {
    async fn notify(&mut self, msg: SmallText, _: &Context<Self>) {
        debug_print!("Received name from Dart: {}", msg.text);
        // Call the gRPC server and forward the response to Dart.
        let url =
            std::env::var("GRPC_SERVER").unwrap_or_else(|_| "http://127.0.0.1:50051".to_owned());

        let fut = say_hello(url, msg.text);

        #[cfg(target_arch = "wasm32")]
        let result = SendWrapper::new(fut).await;

        #[cfg(not(target_arch = "wasm32"))]
        let result = fut.await;

        match result {
            Ok(message) => {
                ServerMessage { text: message }.send_signal_to_dart();
            }
            Err(err) => {
                ServerMessage {
                    text: format!("gRPC error: {}", err),
                }
                .send_signal_to_dart();
            }
        }
    }
}

impl FirstActor {
    /// Listen to an external source, which in this case is Dart.
    async fn listen_to_dart(mut self_addr: Address<Self>) {
        let receiver = SmallText::get_dart_signal_receiver();
        while let Some(signal_pack) = receiver.recv().await {
            let _ = self_addr.notify(signal_pack.message).await;
        }
    }
}
