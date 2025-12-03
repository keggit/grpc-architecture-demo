fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This tells cargo to re-run this script if the proto file changes
    tonic_build::configure()
        .build_server(true) // We need this for the backend
        .build_client(true) // We need this for the frontend
        .build_transport(false)
        .compile_protos(
            &["src/proto/helloworld.proto"], // The path to your .proto file
            &["src/proto"],                  // The directory where protos live
        )?;
    Ok(())
}
