extern crate cbindgen;

use cbindgen::Language;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_include_guard("RUST_LIB_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("rust-lib.h");
}
