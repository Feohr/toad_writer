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
