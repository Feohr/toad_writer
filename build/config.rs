// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.
// SPDX-License-Identifier: BSD-3-Clause
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
// conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of
// conditions and the following disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to
// endorse or promote products derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use serde::Deserialize;
use std::{
    env,
    fs::{read, write},
    path::Path,
};

#[derive(Deserialize)]
pub struct Config {
    pub vars: Vars,
}
#[derive(Deserialize)]
pub struct Vars {
    pub app_name: String,
    pub app_id: String,
    pub base_id: String,
    pub path_id: String,
}

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

pub fn config() {
    match env::var("OUT_DIR") {
        Ok(path) => build_config(path),
        Err(err) => panic!("{}", err),
    }
}

pub fn get_config_table() -> Config {
    let file_content = String::from_utf8(read("tw-config.toml").unwrap()).unwrap();
    let config_table = toml::from_str::<Config>(file_content.as_str())
        .expect("Error while deserializing `tw-config.toml`");

    config_table
}

fn build_config(path: String) {
    let destination = Path::new(&path).clone().join("tw_config.rs");
    let config_table = get_config_table();

    write(
        &destination,
        format!(
            "
            #[allow(unused)]\n\
            pub(crate) const APP_NAME: &'static str = \"{}\";\n\
            #[allow(unused)]\n\
            pub(crate) const APP_ID: &'static str = \"{}\";\n\
            #[allow(unused)]\n\
            pub(crate) const BASE_ID: &'static str = \"{}\";\n\
            #[allow(unused)]\n\
            pub(crate) const PATH_ID: &'static str = \"{}\";\n\
            ",
            config_table.vars.app_name,
            config_table.vars.app_id,
            config_table.vars.base_id,
            config_table.vars.path_id,
        ),
    )
    .unwrap();
}
