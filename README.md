# Rust gRPC + Flutter (Rinf)

This workspace contains a minimal gRPC server and client using `tonic` with a shared protobuf in `proto/`, plus a Flutter UI that talks to the server through a Rust bridge using [Rinf].

- server: Implements `Greeter` service with `SayHello`.
- client: Simple Rust client that calls `SayHello` (CLI demo).
- ui: Flutter app using Rinf; sends a name to Rust, Rust calls the gRPC server, and the server's greeting is displayed in Flutter.

## Layout
- `proto/helloworld.proto`: Shared protobuf schema
- `crates/server`: gRPC server binary crate
- `crates/client`: gRPC client binary crate
- `ui/`: Flutter app with a Rust hub (`ui/native/hub`) using Rinf

## Build
- All Rust crates:
  - `cargo build`

## Run
1) Start the gRPC server (from repo root):
   - `cargo run -p server`

2) Run the Flutter UI (in a new terminal):
   - `cd ui`
   - `flutter run`

3) In the app: enter a name and press “Call gRPC: SayHello”. The server message appears in the UI.

### Alternate: Run the CLI client
- `cargo run -p client -- Alice`
  - Output: `Hello Alice`

## Addresses and env var
- Default server address used by the Rust hub: `http://127.0.0.1:50051`.
- You can override it for desktop targets via environment variable:
  - `GRPC_SERVER=http://127.0.0.1:50051 flutter run` (launched from `ui/`)
  - Or for the CLI client: `GRPC_SERVER=http://127.0.0.1:50051 cargo run -p client -- Bob`
- Mobile emulators/simulators handle localhost differently. For example, on Android Emulator the host machine’s localhost is `10.0.2.2`. You may need to adapt the address in code or add a UI setting to target mobile.

## Codegen
- gRPC stubs: Generated at build time by `tonic-build` from `proto/helloworld.proto`.
  - The Flutter app’s Rust hub (`ui/native/hub/build.rs`) also compiles the shared proto for its client.
- Rinf signals: If you change signal structs in `ui/native/hub/src/signals/mod.rs`, regenerate the Dart bindings (from `ui/`):
  - `cargo install rinf_cli` (once)
  - `rinf gen`

## Rust ↔ Dart flow in the UI
- Dart sends `SmallText { text: String }` to Rust (name input).
- Rust `FirstActor` calls `GreeterClient::say_hello` and sends `ServerMessage { text: String }` back to Dart.
- Flutter subscribes to `ServerMessage.rustSignalStream` and updates the UI.

[Rinf]: https://rinf.cunarist.com
