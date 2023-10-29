// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Library for handling the format of Toad Writer documents.

use gtk::glib::GString;
use std::str::Chars;
use std::iter::Peekable;
use crate::error::TreeError;
use anyhow::Result;

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

#[derive(Debug, PartialEq, Eq)]
pub struct Glyph {
    ty: GlyphType,
    content: Box<str>,
}

impl Glyph {
    fn new(ty: GlyphType, content: String) -> Self {
        Glyph {
            ty,
            content: content.into_boxed_str(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
enum GlyphType {
    Title, // #
    Section, // #!
    Heading(usize), // ##, ###, ####, ...
    Body, // .*
}


pub fn tokenize(buffer: GString) -> Option<Vec<Glyph>> {
    let mut iter = buffer.chars().peekable();
    let mut output = Vec::<Glyph>::new();

    while let Some(ch) = iter.peek() {
        if ch.is_whitespace() && ch.ne(&'\n') {
            iter.next()?;
            continue;
        }

        match ch {
            '#' => {
                let mut appended = match_heading(&mut iter)?;
                output.append(&mut appended);
            },
            _ => output.push(Glyph::new(GlyphType::Body, iter.next()?.to_string())),
        }
    }

    Some(output)
}

fn match_heading<'a>(iter: &mut Peekable<Chars<'a>>) -> Option<Vec<Glyph>> {
    let mut output = Vec::<Glyph>::new();

    let mut level = iter.next()?.to_string();

    while let Some(&'#') = iter.peek() {
        level.push(iter.next()?);
    }

    let mut content = String::new();

    while iter.peek().is_some() {
        let ch = iter.next()?;
        content.push(ch);

        if ch == '\n' { break }
    }

    let content = content.trim().to_string();
    let Ok(glyph) = get_heading_or_title(level, content) else {
        return None;
    };
    output.push(glyph);

    Some(output)
}

fn get_heading_or_title(level: String, content: String) -> Result<Glyph> {
    let len = level.len();

    match len {
        0 => Err(TreeError::NullHeading.into()),
        1 => Ok(Glyph::new(GlyphType::Title, content)),
        _ => Ok(Glyph::new(GlyphType::Heading(len - 1), content)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let txt = "a \tbc\n";

        assert_eq!(
            vec![
                Glyph { ty: GlyphType::Body, content: Box::from("a") },
                Glyph { ty: GlyphType::Body, content: Box::from("b") },
                Glyph { ty: GlyphType::Body, content: Box::from("c") },
                Glyph { ty: GlyphType::Body, content: Box::from("\n") },
            ],
            tokenize(txt.into()).unwrap(),
        );
    }

    #[test]
    fn test_heading() {
        let txt = "a \tb #I am Title\n ### I am heading\n c\n";

        assert_eq!(
            vec![
                Glyph { ty: GlyphType::Body, content: Box::from("a") },
                Glyph { ty: GlyphType::Body, content: Box::from("b") },
                Glyph { ty: GlyphType::Title, content: Box::from("I am Title") },
                Glyph { ty: GlyphType::Heading(2), content: Box::from("I am heading") },
                Glyph { ty: GlyphType::Body, content: Box::from("c") },
                Glyph { ty: GlyphType::Body, content: Box::from("\n") },
            ],
            tokenize(txt.into()).unwrap(),
        );
    }
}
