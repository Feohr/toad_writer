// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.

//! Library for handling the syntax of Toad Writer documents.

use gtk::{prelude::*, TextIter, TextBuffer, TextMark};

const HEADING: char = '#';
const SECTION: char = '~';

trait ParseClosure<T: Sized, F: Fn(&mut TextIter) -> Option<T>> {
    fn parse(&mut self, f: F) -> Option<T>;
}

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

#[derive(Debug, PartialEq, Eq)]
pub struct Glyph {
    ty: GlyphType,
    // mark: Rc<TextMark>,
    content: Box<str>,
}

impl Glyph {
    fn new(ty: GlyphType, /* mark: Rc<TextMark>, */ content: String) -> Self {
        Glyph {
            ty,
            // mark: mark.clone(),
            content: content.into_boxed_str(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
enum GlyphType {
    Title, // #
    Section, // ~
    Heading(usize), // ##, ###, ####, ...
    Body, // .*
}

impl<T: Sized, F: Fn(&mut TextIter) -> Option<T>> ParseClosure<T, F> for TextIter {
    fn parse(&mut self, f: F) -> Option<T> {
        f(self)
    }
}

pub fn tokenize(mark: &TextMark) -> Vec<Glyph> {
    let mut output = Vec::<Glyph>::new();

    if let Some(buffer) = mark.buffer() {
        let iter = buffer.start_iter();
        let mut append = parse_within_bounds(iter);
        output.append(&mut append);
    }

    output
}

fn parse_within_bounds(mut iter: TextIter) -> Vec<Glyph> {
    let mut output = Vec::<Glyph>::new();

    loop {
        let Some(predicate) = iter.parse(skip_whitespaces) else { break };
        let Some(glyph) = (match predicate {
            HEADING => iter.parse(match_heading),
            _ => None,
        }) else { break };

        output.push(glyph);
    }

    output
}

fn match_heading(iter: &mut TextIter) -> Option<Glyph> {
    let mut output = Option::<Glyph>::None;
    let mut body = String::new();

    let mut count = usize::MIN.wrapping_add(1);
    while iter.forward_char() {
        if iter.char().eq(&HEADING) {
            count = count.wrapping_add(1);
            continue
        }
        if iter.ends_line() { break }
        body.push(iter.char());
    }

    if !body.is_empty() {
        let glyph = Glyph::new(GlyphType::Heading(count), body);
        output.replace(glyph);
    }

    output
}

fn skip_whitespaces(iter: &mut TextIter) -> Option<char> {
    while iter.forward_char() {
        let ch = iter.char();
        if !ch.is_whitespace() {
            return Some(ch)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title() {
        gtk::init().unwrap();

        let test = r#"
            # Title
            # Heading 1
            # Heading 2
            "#;

        let buffer = TextBuffer::new(None);
        let mut start = buffer.start_iter();
        buffer.insert(&mut start, test);
        let mark = TextMark::new(Some("all"), true);
        buffer.add_mark(&mark, &start);

        assert_eq!(
            vec![
                Glyph { ty: GlyphType::Heading(1), content: " Title".into()},
                Glyph { ty: GlyphType::Heading(1), content: " Heading 1".into()},
                Glyph { ty: GlyphType::Heading(1), content: " Heading 2".into()},
            ],
            tokenize(&mark),
        );
    }
}
