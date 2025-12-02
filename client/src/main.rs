// The generated code will be included at build time by tonic-build (see build.rs)
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("GRPC_SERVER").unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());
    let name = std::env::args().nth(1).unwrap_or_else(|| "world".to_string());

    let mut client = GreeterClient::connect(addr).await?;

    let request = tonic::Request::new(HelloRequest { name });
    let response = client.say_hello(request).await?;
    println!("{}", response.into_inner().message);
    Ok(())
}

