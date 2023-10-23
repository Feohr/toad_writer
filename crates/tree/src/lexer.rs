// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Library for handling the format of Toad Writer documents.

use gtk::glib::GString;

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
    Title,
    Section,
    Heading,
    Body,
}

pub fn lexer(buffer: GString) -> Vec<Glyph> {
    let iter = buffer.chars();
    let mut output = Vec::<Glyph>::new();

    for ch in iter {
        if ch.is_whitespace() { continue }
        output.push(Glyph::new(GlyphType::Body, ch.to_string()))
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let txt = "a bc";

        assert_eq!(
            vec![
                Glyph { ty: GlyphType::Body, content: Box::from("a") },
                Glyph { ty: GlyphType::Body, content: Box::from("b") },
                Glyph { ty: GlyphType::Body, content: Box::from("c") }
            ],
            lexer(txt.into()),
        );
    }
}
