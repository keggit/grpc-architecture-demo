pub mod api;
mod transport;

pub use api::{ClientError, MessageApi};

pub enum ClientKind {
    Grpc,
}

#[cfg(not(target_arch = "wasm32"))]
pub trait ClientApi: MessageApi + Send + Sync {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: MessageApi + Send + Sync> ClientApi for T {}

#[cfg(target_arch = "wasm32")]
pub trait ClientApi: MessageApi {}
#[cfg(target_arch = "wasm32")]
impl<T: MessageApi> ClientApi for T {}

pub async fn make_client(
    kind: ClientKind,
    endpoint: String,
) -> Result<impl ClientApi, ClientError> {
    match kind {
        ClientKind::Grpc => transport::grpc::GrpcClient::connect(endpoint).await,
    }
}
