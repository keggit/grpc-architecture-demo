# Rust gRPC + Flutter (Rinf)

This workspace demonstrates a full-stack application using **Rust** for the backend (gRPC server) and **Flutter** for the frontend, connected via **Rinf** (Rust in Flutter).

## 🏗️ Architecture

The project is organized into a Cargo workspace with the following components:

| Component | Path | Description |
|-----------|------|-------------|
| **Server** | `crates/server` | A Rust gRPC server using `tonic`. Implements the `Greeter` service. |
| **Client** | `crates/client` | A simple CLI Rust client for testing the server. |
| **Proto** | `crates/proto` | Shared **Protobuf** definitions. Used by Server, Client, and the Flutter App. |
| **UI** | `ui/` | The **Flutter** application. |
| **Hub** | `ui/native/hub` | The Rust "bridge" crate that runs inside the Flutter app. It acts as a gRPC client. |

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
