// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

mod config;
mod gresource;
mod parse_spdx;

use config::config;
use gresource::compile_gresource;
use std::path::PathBuf;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

fn main() {
    config();
    compile_gresource();
    if let Err(err) = parse_spdx::assert_license_identifier(PathBuf::from("./")) {
        panic!("SPDX License Assertion Error: {}", err);
    }
    print!("cargo:rerun-if-changed=build/build.rs");
}
