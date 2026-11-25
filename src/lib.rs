#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use core::num::NonZeroU32;

/// Broad classification for a format diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Category {
    /// Reading or writing the underlying stream failed.
    Io,
    /// The input violates the format's grammar.
    Syntax,
    /// The value is incompatible with the requested type or data model.
    Data,
    /// Input ended before the parser could finish the current construct.
    Eof,
    /// Processing the document would exceed a configured resource limit.
    Limit,
}

/// A location in the original input. Each coordinate can be reported independently.
///
/// Report only known coordinates. A reader may know the byte offset without
/// tracking lines. A Serde backend may supply a line and column without an
/// offset. Use `FormatError::position() == None` when every coordinate is unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    /// Zero-based byte offset from the start of input, including preceding documents.
    /// `Some(0)` is the first byte. An EOF position can equal the input length.
    pub offset: Option<u64>,
    /// One-based line number from the start of input.
    pub line: Option<NonZeroU32>,
    /// One-based byte column from the start of the line.
    /// Use `None` when the backend's column cannot be converted to bytes.
    pub column: Option<NonZeroU32>,
}

/// A parser or writer diagnostic suitable for handling by format-independent callers.
///
/// Implementors must keep input values and sensitive context out of both error
/// formatters and all chained sources. A backend error with arbitrary message
/// text should be classified at the boundary before discarding its text.
pub trait FormatError: core::error::Error {
    /// Classify the error without inspecting its formatted text.
    fn category(&self) -> Category;

    /// Return the known source coordinates, or `None` when no location is available.
    fn position(&self) -> Option<Position>;
}

/// Inclusive, per-document bounds to be enforced by the consuming parser.
///
/// Zero is a real bound on each field. Parsers should check before increasing
/// a counter or allocating storage. Counters must not wrap on overflow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum simultaneously open containers, counting a root container as one.
    pub max_depth: u32,
    /// Maximum encoded input bytes in one document.
    pub max_document_bytes: u64,
    /// Maximum encoded bytes in a token body, including escape sequences and
    /// excluding delimiters. Includes keys, strings, numbers, and comments.
    pub max_token_bytes: usize,
    /// Maximum cumulative expansion cost. Each consuming format documents its
    /// accounting unit. Zero forbids constructs that charge this budget.
    pub max_expansion: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_depth: 128,
            max_document_bytes: 16 * 1024 * 1024,
            max_token_bytes: 1024_usize.saturating_mul(1024),
            max_expansion: 32 * 1024 * 1024,
        }
    }
}
