use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

// IMPORT FROM CRATE
use proto::helloworld::greeter_server::{Greeter, GreeterServer};
use proto::helloworld::{HelloReply, HelloRequest};
use tower_http::cors::{Any, CorsLayer};

#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        println!("Got a request from: {:?}", name); // Added logging to help you debug later
        let reply = HelloReply {
            message: format!("Hello {name}"),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on {addr}");

    let cors = CorsLayer::new()
        .allow_origin(Any) // For dev only. In prod, lock this down!
        .allow_headers(Any)
        .allow_methods(Any);

    // Note: You correctly added .accept_http1(true) and tonic_web::enable
    // This is perfect for the Web support we discussed earlier.
    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(tonic_web::enable(GreeterServer::new(greeter)))
        .serve(addr)
        .await?;

    Ok(())
}
