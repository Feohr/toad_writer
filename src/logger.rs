// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

use anyhow::Result;
use simplelog::{
    format_description, Color, ColorChoice, Config, ConfigBuilder, Level, LevelFilter,
    LevelPadding, TermLogger, TerminalMode,
};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
struct LocalTimeError;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

impl Display for LocalTimeError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Cannot get local time reliably")
    }
}

pub fn init_logger() -> Result<()> {
    init_term_logger()?;
    Ok(())
}

fn config() -> Result<Config> {
    Ok(ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_level_color(Level::Info, Some(Color::Green))
        .set_level_color(Level::Warn, Some(Color::Yellow))
        .set_level_color(Level::Debug, Some(Color::Blue))
        .set_level_color(Level::Error, Some(Color::Red))
        .set_time_format_custom(format_description!("[[[hour]:[minute]:[second]]"))
        .set_time_offset_to_local()
        .or_else(|_| Err(LocalTimeError))?
        .set_location_level(LevelFilter::Error)
        .set_target_level(LevelFilter::Trace)
        .build())
}

#[cfg(not(debug_assertions))]
fn init_term_logger() -> Result<()> {
    Ok(TermLogger::init(
        LevelFilter::Info,
        config()?,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}

#[cfg(debug_assertions)]
fn init_term_logger() -> Result<()> {
    Ok(TermLogger::init(
        LevelFilter::Trace,
        config()?,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}
