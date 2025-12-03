use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, DartSignal)]
pub struct SmallText {
    pub text: String,
}

#[derive(Serialize, RustSignal)]
pub struct ServerMessage {
    pub text: String,
}
