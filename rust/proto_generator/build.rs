use std::path::PathBuf;
#[allow(clippy::unwrap_used)]
fn main() {
    let out_dir = PathBuf::from("../momento-protos/src");
    // tonic-prost-build 0.14 infers a single path type across both arguments, and the
    // proto list is built with `format!`, so the include dir has to be a String too.
    let proto_dir = String::from("../proto");

    eprintln!(
        "Hi brave developer! If you are changing protos and momento-protos fails to build, please retry 1 time."
    );
    eprintln!(
        "Cargo currently does not have a nice way for us to express a dependency order between these 2"
    );
    eprintln!(
        "workspace projects - because this project is _specifically_ supposed to not be a Cargo dependency."
    );
    eprintln!(
        "We did this so downstream users don't need to have protoc when compiling momento-protos!"
    );

    eprintln!(
        "If you are finding that your builds work locally, but not in CI, then you need to manual cleanup some artifacts"
    );
    eprintln!(
        "Clear out the `momento-protos/src` of all protos besides lib.rs, then run `cargo clean` and `cargo build`."
    );

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir.clone())
        .compile_protos(
            &[
                format!("{proto_dir}/permissionmessages.proto"),
                format!("{proto_dir}/auth.proto"),
                format!("{proto_dir}/token.proto"),
                format!("{proto_dir}/cacheclient.proto"),
                format!("{proto_dir}/cachepubsub.proto"),
                format!("{proto_dir}/controlclient.proto"),
                format!("{proto_dir}/leaderboard.proto"),
                format!("{proto_dir}/function_types.proto"),
                format!("{proto_dir}/function.proto"),
            ],
            &[proto_dir.clone()],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir.join("protosocket"))
        .compile_protos(
            &[
                format!("{proto_dir}/protosocket/common.proto"),
                format!("{proto_dir}/protosocket/cache.proto"),
            ],
            &[proto_dir],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protosocket protos {:?}", e));

    println!("cargo:rerun-if-changed=../proto");
}
