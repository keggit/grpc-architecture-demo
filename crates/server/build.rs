fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["../../proto/helloworld.proto"], &["../../proto"]) ?;
    println!("cargo:rerun-if-changed=../../proto/helloworld.proto");
    Ok(())
}
