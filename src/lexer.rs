use crate::token::*;

pub const EOF: char = '\0';

pub struct Lexer<'a> {
    src: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, cursor: 0 }
    }

    pub fn advance_token(&mut self) -> Option<Token> {
        let initial_cursor = self.cursor;

        let first_char = self.bump_char()?;

        let token_kind = match first_char {
            c if c.is_whitespace() => self.whitespace(),
            '_' | 'a'..='z' | 'A'..='Z' => {
                self.eat_ident_or_keyword();
                match &self.src[initial_cursor..self.cursor] {
                    // Section 6.4.1 Keywords
                    "auto" | "break" | "case" | "char" | "const" | "continue" | "default"
                    | "do" | "double" | "else" | "enum" | "extern" | "float" | "for" | "goto"
                    | "if" | "inline" | "int" | "long" | "register" | "restrict" | "return"
                    | "short" | "signed" | "sizeof" | "static" | "struct" | "switch"
                    | "typedef" | "union" | "unsigned" | "void" | "volatile" | "while"
                    | "_Bool" | "_Complex" | "_Imaginary" => Keyword,
                    _ => Ident,
                }
            }
            _c @ '0'..='9' => Const,
            '"' => Literal,
            c if c.is_ascii_punctuation() => Punct,
            _ => Unknown,
        };

        Some(Token::new(
            token_kind,
            initial_cursor..self.cursor,
            self.src,
        ))
    }

    pub fn whitespace(&mut self) -> TokenKind {
        self.eat_whitespace();
        Whitespace
    }

    pub fn eat_whitespace(&mut self) {
        while self.peek_char(0).is_whitespace() && !self.is_eof() {
            self.bump_char();
        }
    }

    pub fn eat_ident_or_keyword(&mut self) {
        while let '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' = self.peek_char(0) {
            self.bump_char();
        }
    }

    pub fn peek_char(&self, nth: usize) -> char {
        self.remaining().chars().nth(nth).unwrap_or(EOF)
    }

    pub fn bump_char(&mut self) -> Option<char> {
        let mut chars = self.remaining().chars();

        let c = chars.next()?;
        self.cursor += self.remaining().len() - chars.as_str().len();
        Some(c)
    }

    pub fn remaining(&self) -> &'a str {
        &self.src[self.cursor..]
    }

    pub fn is_eof(&self) -> bool {
        self.remaining().is_empty()
    }
}
