mod signals;

use client::{make_client, ClientKind, MessageApi};
use rinf::{dart_shutdown, debug_print, write_interface, DartSignal, RustSignal};
use signals::{Message, MessageRequest};

// 1. Generate the Rinf glue code
write_interface!();

// 2. THE ENTRY POINT (Synchronous)
// We use a standard 'fn main' (not async).
// We decide HOW to run async code based on the platform.
pub fn main() {
    // --- NATIVE (Mobile/Desktop) ---
    // We manually build the Tokio Runtime and block on it.
    #[cfg(not(target_arch = "wasm32"))]
    {
        debug_print!("RUST NATIVE: Initializing Tokio Runtime...");
        use tokio::runtime::Builder;

        // Build a multi-threaded runtime for performance
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");

        // "block_on" runs the async function and pauses this thread until it finishes.
        rt.block_on(async_main());
    }

    // --- WEB (WASM) ---
    // We do NOT block. We spawn the future onto the Browser's Event Loop.
    #[cfg(target_arch = "wasm32")]
    {
        debug_print!("RUST WEB: Spawning on JS Event Loop...");
        // This converts the Rust Future into a JS Promise
        wasm_bindgen_futures::spawn_local(async_main());
    }
}

// 3. THE ACTUAL LOGIC (Async)
// This is where your app actually lives.
async fn async_main() {
    debug_print!("RUST: App started.");

    // Spawn the logic loop (it runs concurrently)
    // We need to use platform-specific spawning here too
    spawn_logic_loop();

    // Wait for Dart to tell us to quit
    dart_shutdown().await;
}

fn spawn_logic_loop() {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::spawn(logic_loop());

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(logic_loop());
}

async fn logic_loop() {
    debug_print!("RUST: Logic loop listening...");
    let receiver = MessageRequest::get_dart_signal_receiver();

    while let Some(signal) = receiver.recv().await {
        spawn_grpc_request(signal.message);
    }
}

fn spawn_grpc_request(message_request: MessageRequest) {
    // Use localhost for Web, or 10.0.2.2 for Android Emulator
    let url = "http://localhost:50051".to_string();

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(run_request(url, message_request));

    #[cfg(not(target_arch = "wasm32"))]
    tokio::spawn(run_request(url, message_request));
}

async fn run_request(url: String, message_request: MessageRequest) {
    debug_print!("RUST: Sending gRPC...");

    let client_res = make_client(ClientKind::Grpc, url).await;

    match client_res {
        Ok(client) => match client.get_message(message_request.into()).await {
            Ok(msg) => {
                debug_print!("RUST: Success -> {}", msg.text);
                Message::from(msg).send_signal_to_dart();
            }
            Err(e) => {
                debug_print!("RUST: Error -> {:?}", e);
                Message {
                    text: format!("Error: {e}"),
                }
                .send_signal_to_dart()
            }
        },
        Err(e) => {
            debug_print!("RUST: Client creation error -> {:?}", e);
            Message {
                text: format!("Error: {e}"),
            }
            .send_signal_to_dart()
        }
    }
}
