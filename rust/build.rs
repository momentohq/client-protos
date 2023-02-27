use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "proto/google/rpc/code.proto",
                "proto/auth.proto",
                "proto/cacheclient.proto",
                "proto/cachepubsub.proto",
                "proto/controlclient.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    Ok(())
}
