use crate::api::{ClientError, MessageApi};
use async_trait::async_trait;
use domain::{Message, MessageRequest};
use proto::helloworld::greeter_client::GreeterClient;
use proto::helloworld::HelloRequest;

#[cfg(not(target_arch = "wasm32"))]
type Transport = tonic::transport::Channel;

#[cfg(target_arch = "wasm32")]
type Transport = tonic_web_wasm_client::Client;

#[derive(Clone)]
pub struct GrpcClient {
    inner: GreeterClient<Transport>,
}

impl GrpcClient {
    pub async fn connect(endpoint: String) -> Result<Self, ClientError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let channel = tonic::transport::Endpoint::from_shared(endpoint)
                .map_err(|e| ClientError::Network(e.to_string()))?
                .connect()
                .await
                .map_err(|e| ClientError::Network(e.to_string()))?;
            Ok(Self {
                inner: GreeterClient::new(channel),
            })
        }

        #[cfg(target_arch = "wasm32")]
        {
            let client = tonic_web_wasm_client::Client::new(endpoint);
            Ok(Self {
                inner: GreeterClient::new(client),
            })
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl MessageApi for GrpcClient {
    async fn get_message(&self, request: MessageRequest) -> Result<Message, ClientError> {
        // Map domain -> proto
        let req: HelloRequest = request.into();

        let mut client = self.inner.clone();
        let resp = client
            .say_hello(req)
            .await
            .map_err(|e| ClientError::Network(e.to_string()))?
            .into_inner();

        // Map proto -> domain
        Ok(resp.into())
    }
}
