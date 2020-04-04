use crate::source::SourceFile;
use crate::token::*;
use std::fmt;

#[derive(Debug, Default)]
pub struct Statistics {
    pub lines: usize,
    pub len: usize,
    pub keywords: usize,
    pub idents: usize,
    pub floats: usize,
    pub ints: usize,
    pub chars: usize,
    pub strs: usize,
    pub puncts: usize,
    pub errors: usize,
}

impl Statistics {
    pub fn new(source: &SourceFile) -> Self {
        let lines = source.lines.len();
        let len = source.src.chars().count();
        Self {
            lines,
            len,
            ..Self::default()
        }
    }

    pub fn track(&mut self, token: &Token) {
        match token.kind {
            Keyword => self.keywords += 1,
            Ident => self.idents += 1,
            Const(Float) => self.floats += 1,
            Const(Integer) => self.ints += 1,
            Const(Char) => self.chars += 1,
            StrLit => self.strs += 1,
            Punct => self.puncts += 1,
            Error(_) => self.errors += 1,
            _ => (),
        }
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   Total lines: {}", self.lines)?;
        writeln!(f, "   Total chars: {}", self.len)?;
        writeln!(f, "   Keywords: {}", self.keywords)?;
        writeln!(f, "   Identifiers: {}", self.idents)?;
        writeln!(f, "   Floating constants: {}", self.floats)?;
        writeln!(f, "   Integer constants: {}", self.ints)?;
        writeln!(f, "   Char constants: {}", self.chars)?;
        writeln!(f, "   String literals: {}", self.strs)?;
        writeln!(f, "   Punctuators: {}", self.puncts)?;
        writeln!(f, "   Errors: {}", self.errors)?;
        Ok(())
    }
}
