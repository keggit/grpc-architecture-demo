use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct MessageRequest {
    pub name: String,
}

#[derive(Serialize, RustSignal)]
pub struct Message {
    pub text: String,
}

impl From<MessageRequest> for domain::MessageRequest {
    fn from(request: MessageRequest) -> Self {
        domain::MessageRequest { name: request.name }
    }
}

impl From<domain::Message> for Message {
    fn from(message: domain::Message) -> Self {
        Message { text: message.text }
    }
}
