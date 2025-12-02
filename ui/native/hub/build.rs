fn main() {
    // Compile the shared protobuf for the gRPC client.
    // Path relative to this crate: ui/native/hub -> repo root -> proto
    tonic_build::configure()
        .build_server(false)
        .compile(&["../../../proto/helloworld.proto"], &["../../../proto"]) // proto dir includes
        .expect("failed to compile protos");
    println!("cargo:rerun-if-changed=../../../proto/helloworld.proto");
}

