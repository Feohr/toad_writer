// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

pub type SPDXResult = Result<(), Box<dyn std::error::Error>>;

// Directories that should be ignored
const IGNORE_DIR: [&'static str; 6_usize] = ["target", "..", "/", "docs", "data", ".git"];

#[derive(Debug, Error)]
#[error(
    "The license lines need to be added at the top of the file.\n\
    Expected: \"SPDX-License-Identifier: BSD-3-Clause \
    Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.\"\n\
    Found: {0:?} at {1:?}."
)]
struct SPDXErr(String, String);

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

pub fn assert_license_identifier(path: PathBuf) -> SPDXResult {
    for file in std::fs::read_dir(path)? {
        // Get file path
        let file = file?;
        let path = file.path();
        let path_name = path
            .file_name()
            .unwrap_or(OsStr::new("NoName"))
            .to_string_lossy()
            .to_string();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("NoExt");

        // If the path is a file
        if !path.is_dir() && extension.eq("rs") {
            let file = std::fs::read_to_string(path.clone())?;
            parse_license_file(file, path_name)?;
        // If the path is a directory
        } else if path.is_dir() && !IGNORE_DIR.contains(&path_name.as_str()) {
            assert_license_identifier(path.clone())?;
        }
        // Do nothing if not both
    }

    Ok(())
}

// Let's keep it real simple...
fn parse_license_file(file: String, name: String) -> SPDXResult {
    let lines = file
        .lines()
        .skip_while(|line| line.is_empty())
        .take(2_usize)
        .map(|line| line.trim_start_matches("//").trim())
        .collect::<Vec<&str>>();
    let lines = (lines[0_usize], lines[1_usize]);

    // This works just fine
    // Why should I spend anymore time on this?
    if (
        "SPDX-License-Identifier: BSD-3-Clause",
        "Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.",
    ) != lines
    {
        return Err(Box::new(SPDXErr(format!("{} {}", lines.0, lines.1), name)));
    }

    Ok(())
}
