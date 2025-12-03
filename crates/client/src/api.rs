use async_trait::async_trait;
use domain::{Message, MessageRequest};

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("network error: {0}")]
    Network(String),
    #[error("server error: {0}")]
    Server(String),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait MessageApi {
    async fn get_message(&self, request: MessageRequest) -> Result<Message, ClientError>;
}
