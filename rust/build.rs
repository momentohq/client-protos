use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/")
        .compile(
            &["proto/controlclient.proto", "proto/cacheclient.proto", "proto/momento/auth.proto"],
            &["proto", "proto/momento"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    Ok(())
}
