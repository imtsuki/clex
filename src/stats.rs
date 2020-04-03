use crate::token::*;
use std::fmt;

#[derive(Debug, Default)]
pub struct Statistics {
    pub keywords: u32,
    pub idents: u32,
    pub floats: u32,
    pub ints: u32,
    pub chars: u32,
    pub strs: u32,
    pub puncts: u32,
    pub errors: u32,
}

impl Statistics {
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
