use std::fmt;
use std::ops::Range;
pub use ConstKind::*;
pub use ErrorKind::*;
pub use TokenKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    /* tokens */
    Keyword,
    Ident,
    Const(ConstKind),
    StrLit,
    Punct,
    /* compiler internal tokens */
    Comment,
    Whitespace,
    Error(ErrorKind),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstKind {
    Float,
    Integer,
    Char,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    UnclosedBlockComment,
    UnterminatedString,
    UnterminatedChar,
    UnknownPunctuator,
    UnexpectedCharacter,
    InvalidIntegerSuffix,
    InvalidFloatingSuffix,
    NoHexadecimalDigits,
    #[allow(unused)]
    Unknown,
}

/// Instead of storing the actual token content, `Token` stores
/// the text range of it and maintains a reference to the source str.
///
/// By doing so, we can avoid additional heap allocations.
///
/// Note that we have two ranges: one is byte-based, corresponding to
/// the original UTF-8 byte sequence; the other is char-based, mainly
/// for human-readable text ranges.
#[derive(Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub byte_range: Range<usize>,
    pub char_range: Range<usize>,
    pub src: &'a str,
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}@{:?}]", self.kind, self.byte_range)
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?}: {:?}]",
            self.kind,
            &self.src[self.byte_range.clone()],
        )
    }
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        byte_range: Range<usize>,
        char_range: Range<usize>,
        src: &'a str,
    ) -> Self {
        Token {
            kind,
            byte_range,
            char_range,
            src,
        }
    }
}
