// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use anyhow::Result;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

pub type SPDXResult<T> = Result<T>;

// Directories that should be ignored
const IGNORE_DIR: [&str; 6_usize] = ["target", "..", "/", "docs", "data", ".git"];

// Main error object
#[derive(Debug, Error)]
#[error("Error while parsing the SPDX comments in ({file}).")]
struct ParseCommentError {
    file: String,
    #[source]
    error: SPDXError,
}

// Error types
#[derive(Debug, Error)]
#[error("Please make sure the top SPDX comments are added to your file.")]
enum SPDXError {
    #[error("The SPDX license line is not added to the top of your file.")]
    SPDXLineNotAdded,
    #[error("The license type is not correct found {0:?}.")]
    IncorrectLicenseType(String),
    #[error("Incorrect copyright information found {0:?}.")]
    IncorrectCopyrightInfo(String),
    #[error(
        "Improper SPDX license line format. Expected `SPDX-License-Identifier: (licenes type)` \
        but found {0:?}"
    )]
    ImproperLicenseFormat(String),
}

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

impl ParseCommentError {
    pub fn new(file: String, error: SPDXError) -> Self {
        Self { file, error }
    }
}

pub fn assert_license_identifier(path: PathBuf) -> SPDXResult<()> {
    for file in std::fs::read_dir(path)? {
        dbg!(&file);
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
fn parse_license_file(file: String, name: String) -> SPDXResult<()> {
    let lines = file
        .lines()
        .filter(|line| !line.is_empty())
        .take(2_usize)
        .filter(|line| line.starts_with("//"))
        .map(|line| line.trim_start_matches("//").trim())
        .collect::<Vec<&str>>();
    // Check if top comments even exists
    let (line1, line2) = comment_line_empty_check(name.clone(), lines)?;

    // Comment Validations
    check_first_comment(name.clone(), line1)?;

    if !line2.starts_with("Copyright")
        || "(Feohr) Mohammed Rehaan and the ToadWriter contributors"
            .split_whitespace()
            .map(|token| line2.contains(token))
            .collect::<Vec<bool>>()
            .contains(&false)
    {
        return Err(ParseCommentError::new(
            name,
            SPDXError::IncorrectCopyrightInfo(line2.to_string()),
        )
        .into());
    }
    Ok(())
}

#[inline]
fn comment_line_empty_check(name: String, lines: Vec<&str>) -> SPDXResult<(&str, &str)> {
    if let (Some(line1), Some(line2)) = (lines.first(), lines.get(1_usize)) {
        if !line1.is_empty() && !line2.is_empty() {
            return Ok((line1, line2));
        }
    }

    return Err(ParseCommentError::new(name, SPDXError::SPDXLineNotAdded).into());
}

fn check_first_comment(name: String, line: &str) -> SPDXResult<()> {
    if line.starts_with("SPDX-License-Identifier:") {
        if line.split_once(':').unwrap().1.trim() != "BSD-3-Clause" {
            return Err(ParseCommentError::new(
                name,
                SPDXError::IncorrectLicenseType(line.to_string()),
            )
            .into());
        }
        return Ok(());
    }

    return Err(
        ParseCommentError::new(name, SPDXError::ImproperLicenseFormat(line.to_string())).into(),
    );
}
