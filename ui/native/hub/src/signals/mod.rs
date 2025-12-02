use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};

/// To send data from Dart to Rust, use `DartSignal`.
#[derive(Deserialize, DartSignal)]
pub struct SmallText {
    pub text: String,
}

/// Message coming back from the gRPC server.
#[derive(Serialize, RustSignal)]
pub struct ServerMessage {
    pub text: String,
}
