use std::fmt;
use std::ops::Range;
pub use TokenKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Keyword,
    Ident,
    Const,
    Literal,
    Punct,
    Whitespace,
    Unknown,
}

/// Instead of storing the actual token content, `Token` stores
/// the text range of it and maintains a reference to the source str.
///
/// By doing so, we can avoid additional heap allocations.
#[derive(Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub range: Range<usize>,
    pub src: &'a str,
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"[{:?} {:?} @{:?}]"#,
            self.kind,
            &self.src[self.range.clone()],
            self.range
        )
    }
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, range: Range<usize>, src: &'a str) -> Self {
        Token { kind, range, src }
    }
}
