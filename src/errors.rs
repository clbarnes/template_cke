use thiserror::Error;

/// Errors produced while parsing a format string and constructing an
/// [`Interpolator`](crate::Interpolator).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    /// A brace was unmatched in the format string (an unescaped `{` or `}`).
    #[error("Unmatched brace in format string: {0}")]
    UnmatchedBrace(String),
    /// The pad specifier after `:` was not a valid padding number.
    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),
    /// An index could not be parsed as an integer.
    #[error("Invalid index: {0}")]
    InvalidIndex(String),
    /// The same index was referenced by more than one placeholder.
    #[error("Index {0} is already used")]
    DuplicateIndex(isize),
    /// A catch-all placeholder (`{*}`) was used without supplying a separator.
    #[error("Catch-all part requires a separator")]
    CatchAllNeedsSeparator,
    /// More than one catch-all placeholder (`{*}`) appeared in the format.
    #[error("Cannot have multiple catch-all parts")]
    MultipleCatchAll,
}

/// Errors produced while interpolating chunk indices into a format string.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InterpolateError {
    /// A referenced index was out of bounds for the given chunk indices.
    #[error("Index {0} is out of bounds for chunk index of length {1}")]
    IndexOutOfBounds(isize, usize),
    /// Writing into the output buffer failed.
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
}
