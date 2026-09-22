use super::Part;
use crate::errors::ParseError;
use std::iter::Peekable;
use std::str::Chars;

struct FormatParser<'a> {
    parts: Vec<Part>,
    in_brace: bool,
    buf: String,
    format: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> FormatParser<'a> {
    fn new(format: &'a str) -> Self {
        Self {
            parts: Vec::new(),
            in_brace: false,
            buf: String::with_capacity(format.len()),
            format,
            chars: format.chars().peekable(),
        }
    }

    fn take_part(&mut self) -> Result<Option<Part>, ParseError> {
        self.buf.clear();
        while let Some(c) = self.chars.next() {
            if self.in_brace && c == '}' {
                self.in_brace = false;
                return Some(self.buf.parse()).transpose();
            } else if !self.in_brace && c == '{' {
                if self.chars.peek() == Some(&'{') {
                    self.buf.push('{');
                    self.chars.next();
                    continue;
                }
                self.in_brace = true;
                return Ok(Some(Part::String(self.buf.clone())));
            } else if !self.in_brace && c == '}' {
                if self.chars.peek() == Some(&'}') {
                    self.buf.push('}');
                    self.chars.next();
                    continue;
                }
                return Err(ParseError::UnmatchedBrace(self.format.to_string()));
            } else {
                self.buf.push(c);
            }
        }
        if self.in_brace {
            Err(ParseError::UnmatchedBrace(self.format.to_string()))
        } else if self.buf.is_empty() {
            Ok(None)
        } else {
            let out = Some(Part::String(self.buf.clone()));
            self.buf.clear();
            Ok(out)
        }
    }

    fn into_parts(self) -> Vec<Part> {
        self.parts
    }
}

pub fn parse_parts(format: &str) -> Result<Vec<Part>, ParseError> {
    let mut parser = FormatParser::new(format);
    while let Some(part) = parser.take_part()? {
        parser.parts.push(part);
    }
    Ok(parser.into_parts())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{FORMAT, expected};

    #[test]
    fn can_parse_parts() {
        let res = parse_parts(FORMAT).unwrap();
        assert_eq!(res, expected());
    }
}
