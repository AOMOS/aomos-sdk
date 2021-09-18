use std::path::PathBuf;
use std::str::FromStr;

use glob::glob;

fn main() {
    let build_server = std::env::var("CARGO_FEATURE_SERVER").is_ok();

    let protos: Vec<PathBuf> = glob("../../protobufs/**/*.proto")
        .unwrap()
        .map(|path| path.unwrap())
        .collect();

    tonic_build::configure()
        .build_server(build_server)
        .compile(&protos, &[PathBuf::from_str("../../protobufs").unwrap()])
        .unwrap();
}
