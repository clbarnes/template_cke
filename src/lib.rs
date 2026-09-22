use std::collections::BTreeMap;
use std::fmt::Write;
use std::str::FromStr;

use crate::errors::{InterpolateError, ParseError};
use crate::parser::parse_parts;
pub mod errors;
pub mod parser;

#[cfg(feature = "zarrs")]
pub mod zarrs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    String(String),
    Index { idx: usize, pad: usize },
    NegIndex { idx: isize, pad: usize },
    CatchAll { pad: usize },
}

impl Part {
    fn padded_to(&self) -> usize {
        match self {
            Part::Index { pad, .. } => *pad,
            Part::NegIndex { pad, .. } => *pad,
            Part::CatchAll { pad } => *pad,
            Part::String(_s) => 0,
        }
    }
}

impl From<isize> for Part {
    fn from(index: isize) -> Self {
        if index < 0 {
            Self::NegIndex { idx: index, pad: 0 }
        } else {
            Self::Index {
                idx: index as usize,
                pad: 0,
            }
        }
    }
}

fn parse_number_format(s: &str) -> Result<usize, ParseError> {
    if !s.starts_with('0') {
        return Err(ParseError::InvalidNumberFormat(s.to_string()));
    }
    s[1..]
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidNumberFormat(s.to_string()))
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (idx_str, pad) = if let Some((pre, post)) = s.split_once(':') {
            (pre, parse_number_format(post)?)
        } else {
            (s, 0)
        };
        if idx_str == "*" {
            Ok(Part::CatchAll { pad })
        } else if idx_str.starts_with('-') {
            Ok(Part::NegIndex {
                idx: idx_str
                    .parse::<isize>()
                    .map_err(|_| ParseError::InvalidIndex(idx_str.to_string()))?,
                pad,
            })
        } else {
            Ok(Part::Index {
                idx: idx_str
                    .parse::<usize>()
                    .map_err(|_| ParseError::InvalidIndex(idx_str.to_string()))?,
                pad,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpolator {
    parts: Vec<Part>,
    pad_by_idx: BTreeMap<isize, usize>,
    sep: String,
    strs_len: usize,
    max_pad: usize,
}

impl Interpolator {
    pub fn try_new(fmt: &str, sep: Option<impl Into<String>>) -> Result<Self, ParseError> {
        let parts = parse_parts(fmt)?;
        Self::from_parts(parts, sep)
    }

    pub(crate) fn from_parts(
        parts: Vec<Part>,
        sep: Option<impl Into<String>>,
    ) -> Result<Self, ParseError> {
        let mut pad_by_idx = BTreeMap::default();
        let mut has_catchall = false;
        let mut strs_len: usize = 0;
        let mut max_pad: usize = 0;

        for part in &parts {
            max_pad = max_pad.max(part.padded_to());
            match part {
                Part::Index { idx, pad } => {
                    max_pad = max_pad.max(*pad);
                    if pad_by_idx.insert(*idx as isize, *pad).is_some() {
                        return Err(ParseError::DuplicateIndex(*idx as isize));
                    }
                }
                Part::NegIndex { idx, pad } => {
                    max_pad = max_pad.max(*pad);
                    if pad_by_idx.insert(*idx, *pad).is_some() {
                        return Err(ParseError::DuplicateIndex(*idx));
                    }
                }
                Part::CatchAll { pad } => {
                    if sep.is_none() {
                        return Err(ParseError::CatchAllNeedsSeparator);
                    }
                    max_pad = max_pad.max(*pad);
                    if has_catchall {
                        return Err(ParseError::MultipleCatchAll);
                    }
                    has_catchall = true;
                }
                Part::String(s) => {
                    strs_len += s.len();
                }
            }
        }

        Ok(Self {
            parts,
            pad_by_idx,
            sep: sep.map(Into::into).unwrap_or_default(),
            strs_len,
            max_pad,
        })
    }

    pub fn has_catchall(&self) -> bool {
        self.parts
            .iter()
            .any(|p| matches!(p, Part::CatchAll { .. }))
    }

    /// Suggest a length (in bytes) for the string buffer to allocate in [Self::interpolate].
    fn buf_len(&self, chunk_idx: &[u64]) -> usize {
        let max_digits = chunk_idx
            .iter()
            .max()
            .map(|i| if i > &0 { i.ilog10() + 1 } else { 1 })
            .unwrap_or(0) as usize;
        max_digits.max(self.max_pad) * chunk_idx.len()
            + self.strs_len
            + (chunk_idx.len().saturating_sub(self.pad_by_idx.len() + 1)) * self.sep.len()
    }

    pub fn interpolate(&self, chunk_idx: &[u64]) -> Result<String, InterpolateError> {
        let mut out = String::with_capacity(self.buf_len(chunk_idx));

        for part in &self.parts {
            match part {
                Part::String(s) => out.push_str(s),
                Part::Index { idx, pad } => {
                    let arg = chunk_idx
                        .get(*idx)
                        .ok_or(InterpolateError::IndexOutOfBounds(
                            *idx as isize,
                            chunk_idx.len(),
                        ))?;
                    write!(out, "{arg:0pad$}", pad = pad)?;
                }
                Part::NegIndex { idx, pad } => {
                    let pos_idx = chunk_idx.len() as isize + idx;

                    if pos_idx >= 0 {
                        let arg = chunk_idx
                            .get(pos_idx as usize)
                            .ok_or(InterpolateError::IndexOutOfBounds(*idx, chunk_idx.len()))?;
                        write!(out, "{arg:0pad$}", pad = pad)?;
                    } else {
                        return Err(InterpolateError::IndexOutOfBounds(*idx, chunk_idx.len()));
                    }
                }
                Part::CatchAll { pad } => {
                    let mut first = true;
                    for (i, arg) in chunk_idx.iter().enumerate() {
                        let neg_arg = i as isize - chunk_idx.len() as isize;
                        if self.pad_by_idx.contains_key(&neg_arg)
                            || self.pad_by_idx.contains_key(&(i as isize))
                        {
                            continue;
                        }
                        if first {
                            first = false;
                        } else {
                            out.push_str(&self.sep);
                        }
                        write!(out, "{arg:0pad$}", pad = pad)?;
                    }
                }
            }
        }

        Ok(out)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub const FORMAT: &str = "potato{0}-{2}/{4:03}_{*},{-1}suffix";

    pub fn expected() -> Vec<Part> {
        vec![
            Part::String("potato".into()),
            Part::Index { idx: 0, pad: 0 },
            Part::String("-".into()),
            Part::Index { idx: 2, pad: 0 },
            Part::String("/".into()),
            Part::Index { idx: 4, pad: 3 },
            Part::String("_".into()),
            Part::CatchAll { pad: 0 },
            Part::String(",".into()),
            Part::NegIndex { idx: -1, pad: 0 },
            Part::String("suffix".into()),
        ]
    }

    fn make_interp() -> Interpolator {
        Interpolator::try_new(FORMAT, Some(":")).unwrap()
    }

    #[test]
    fn can_instantiate() {
        let interpolator = make_interp();
        assert_eq!(interpolator.parts, expected());
    }

    #[test]
    fn can_interpolate() {
        let interpolator = make_interp();
        let result = interpolator.interpolate(&[0, 1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(result, "potato0-2/004_1:3:5,6suffix");
    }
}
