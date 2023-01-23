use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR env var was not set");

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(
            &[
                "proto/controlclient.proto",
                "proto/cacheclient.proto",
                "proto/auth.proto",
            ],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    // #[path = ".."] attributes don't allow us to specify paths relative to
    // the OUT_DIR environment variable. To work around this we generate a mod.rs
    // file in OUT_DIR and use include! within lib.rs.
    std::fs::copy("src/lib.tmpl.rs", PathBuf::from(out_dir).join("mod.rs"))?;

    Ok(())
}
