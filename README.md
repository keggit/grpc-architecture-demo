# Rust gRPC + Flutter (Rinf)

This workspace demonstrates a full-stack application using **Rust** for the backend (gRPC server) and **Flutter** for the frontend, connected via **Rinf** (Rust in Flutter).

## 🏗️ Architecture

The project is organized into a Cargo workspace with the following components:

| Component | Path | Description |
|-----------|------|-------------|
| **Domain** | `crates/domain` | **Canonical Data Models**. Pure Rust structs used by the UI and Server logic. |
| **Proto** | `crates/proto` | **Transport Layer**. Defines Protobufs and implements `From`/`Into` conversions to Domain types. |
| **Client** | `crates/client` | **Façade**. Exposes a clean `MessageApi` using Domain types. Hides transport details (gRPC/HTTP/WASM). |
| **Server** | `crates/server` | A Rust gRPC server. Uses `domain` types for logic and converts to `proto` at the boundary. |
| **UI** | `ui/` | The **Flutter** application. |
| **Hub** | `ui/native/hub` | The Rust "bridge" crate. Uses `client` to fetch data and `domain` types to communicate with Dart. |

## 🧠 Domain-Driven Design & Conversion

This project follows a strict separation of concerns:

1.  **Domain is King**: `crates/domain` contains the "truth". All business logic in the Server and UI uses these types.
2.  **Proto is just Transport**: `crates/proto` depends on `domain`. It implements `From<Domain> for Proto` and `From<Proto> for Domain`.
3.  **Seamless Conversion**:
    -   **Client**: Calls `request.into()` to send proto, and `response.into()` to return domain objects.
    -   **Server**: Calls `request.into_inner().into()` to get domain objects, and returns `response.into()` to send proto.

### Example Flow

```rust
// 1. UI/Hub creates a Domain Request
let req = domain::MessageRequest { name: "World".to_string() };

// 2. Client converts to Proto automatically
let proto_req: proto::HelloRequest = req.into(); 

// 3. Server receives Proto, converts back to Domain
let domain_req: domain::MessageRequest = proto_req.into();

// 4. Server logic produces Domain Response
let domain_msg = domain::Message { text: "Hello World".to_string() };

// 5. Server converts to Proto Response
let proto_msg: proto::HelloReply = domain_msg.into();

// 6. Client receives Proto, converts back to Domain
let result: domain::Message = proto_msg.into();
```

---

## 🚀 Getting Started

### 1. Run the gRPC Server
The server must be running for the app to work.
```bash
# From the project root
cargo run --bin server
```
*The server listens on `http://127.0.0.1:50051` by default.*

### 2. Run the Flutter App

#### 📱 Native (iOS / Android / macOS / Windows)
For native platforms, Rinf automatically handles the Rust compilation.
```bash
cd ui
flutter run
```
*Note: On Android Emulator, `localhost` is `10.0.2.2`. The code currently defaults to `127.0.0.1`, so you may need to adjust `ui/native/hub/src/lib.rs` for Android.*

#### 🌐 Web (WASM)
The Web build requires a specific build step to avoid browser compatibility issues (specifically regarding shared memory/atomics).

**Step 1: Build the WASM module**
Use the provided script to build the Rust logic for the web:
```bash
# From the project root
./ui/build_wasm.sh
```

**Step 2: Run Flutter Web**
```bash
cd ui
flutter run -d chrome --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```
*(The headers are required for Rinf's communication channel, even without atomics)*

---

## 🛠️ Development Workflow

### Modifying Protobufs
1. Edit `crates/proto/helloworld.proto`.
2. The Rust code (`server`, `client`, `hub`) will automatically pick up changes on the next `cargo build`.
3. **Note**: If you change the service definition, you may need to update the Rust implementation in `crates/server` and `ui/native/hub`.

### Modifying Rinf Messages
The communication between Dart and Rust (inside the app) is defined by Rinf messages.
1. Edit `ui/native/hub/src/signals/mod.rs` (or other message files).
2. Regenerate the Dart glue code:
   ```bash
   cd ui
   rinf gen
   ```
3. If targeting Web, rebuild the WASM:
   ```bash
   ../ui/build_wasm.sh
   ```

## 🐛 Troubleshooting

**"TypeError: [object Int32Array] is not a shared typed array"**
- This happens if the WASM is built *with* atomics enabled but run in a browser environment that doesn't support them or is missing headers.
- **Fix**: Always use `./ui/build_wasm.sh` to build for web. Do **not** use `rinf wasm` directly, as it forces atomics on.
