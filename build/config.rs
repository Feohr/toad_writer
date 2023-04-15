// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

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
