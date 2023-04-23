// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SPDXParseErr {
    #[error("The given license is not BSD License. Found at {0:?}")]
    NotBSDLicense(String),
    #[error("The SPDX license comment needs to be the first thing in a file. Found at {0:?}")]
    LicenseNotFirst(String),
    #[error(
        "Cannot parse the SPDX identifier. \
    Please add at the top of the file if there isn't one. Found at {0:?}"
    )]
    ParsingError(String),
}

pub type SPDXResult<T> = Result<T, Box<dyn std::error::Error>>;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

pub fn assert_license_identifier() -> SPDXResult<()> {
    // Assert inside source directory
    let path = PathBuf::from("src");
    _assert_license_identifier(path)?;
    // Assert inside build directory
    let path = PathBuf::from("build");
    _assert_license_identifier(path)?;
    Ok(())
}

fn _assert_license_identifier(path: PathBuf) -> SPDXResult<()> {
    for file in std::fs::read_dir(path)? {
        dbg!(&file);

        // Get file path
        let file = file?;
        let path = file.path();

        // If the path is a directory
        if path.is_dir() {
            _assert_license_identifier(path.clone())?;
            continue;
        }
        // If the path is a file
        let file = std::fs::read_to_string(path.clone())?;
        parse_license_file(
            file,
            path.file_name()
                .unwrap_or(OsStr::new("NoName"))
                .to_string_lossy()
                .to_string(),
        )?;
    }
    Ok(())
}

fn parse_license_file(file: String, name: String) -> SPDXResult<()> {
    // Skipping the first n empty lines
    // If file is empty, don't do license assertion
    if let Some(line) = file
        .lines()
        .skip_while(|line| line.is_empty())
        .map(|line| line.trim_matches('\x20'))
        .collect::<Vec<&str>>()
        .first()
    {
        // The first line is not a comment
        if !line.starts_with("//") {
            return Err(Box::new(SPDXParseErr::LicenseNotFirst(name)));
        }
        // Trim the comment tags
        let line = line.trim_matches('/');

        // Get the tokens
        let mut split = line
            .split_whitespace()
            .flat_map(|split| split.split(':'))
            .filter(|token| !token.is_empty())
            .collect::<Vec<&str>>();
        split.reverse();
        dbg!(&split);

        // Get the license ID
        let id = get_license_id(split, name.clone())?;
        // Assertion for correct ID
        if id.ne("BSD-3-Clause") {
            return Err(Box::new(SPDXParseErr::NotBSDLicense(name)));
        }
    }
    Ok(())
}

fn get_license_id<'a>(mut split: Vec<&'a str>, name: String) -> SPDXResult<&'a str> {
    // If token size is exactly two
    if split.len() == 2_usize && Some("SPDX-License-Identifier") == split.pop() {
        if let Some(id) = split.pop() {
            return Ok(id);
        }
    }
    return Err(Box::new(SPDXParseErr::ParsingError(name)));
}
