# Rust gRPC Sample (tonic)

This workspace contains a minimal gRPC server and client using `tonic` with a shared protobuf in `proto/`.

- server: Implements `Greeter` service with `SayHello`.
- client: Connects to the server and calls `SayHello`.

## Layout
- `proto/helloworld.proto`: Shared protobuf schema
- `crates/server`: gRPC server binary crate
- `crates/client`: gRPC client binary crate

## Build
cargo build

## Run
- Start the server:
  cargo run -p server

- In another terminal, run the client (optional name arg):
  cargo run -p client -- Alice
  # -> Hello Alice

- Override server address for the client via env var:
  GRPC_SERVER=http://127.0.0.1:50051 cargo run -p client -- Bob

## Regenerating code
Code is generated at build time by `tonic-build` from `proto/helloworld.proto` via each crate's `build.rs`. Edit the proto and rebuild to regenerate.
