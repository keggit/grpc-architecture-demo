// Use the shared crate, don't include_proto! again
use proto::helloworld::greeter_client::GreeterClient;
use proto::helloworld::HelloRequest;

pub async fn say_hello(url: String, name: String) -> Result<String, Box<dyn std::error::Error>> {
    // 1. WASM Transport
    /*e proto::helloworld::HelloRequest;
    code is inactive due to #[cfg] directives: target_arch = "wasm32" is disabledrust-analyzerinactive-code
     */
    #[cfg(target_arch = "wasm32")]
    let channel = tonic_web_wasm_client::Client::new(url);

    // 2. Native Transport
    #[cfg(not(target_arch = "wasm32"))]
    let channel = tonic::transport::Endpoint::from_shared(url)?
        .connect()
        .await?;

    // 3. Unified Client Logic
    // GreeterClient accepts either transport because both impl Service
    let mut client = GreeterClient::new(channel);

    let request = tonic::Request::new(HelloRequest { name });
    let response = client.say_hello(request).await?;

    Ok(response.into_inner().message)
}
