use crate::token::*;

pub const EOF: char = '\0';

pub struct Lexer<'a> {
    src: &'a str,
    byte_cursor: usize,
    char_cursor: usize,
    prev: char,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            byte_cursor: 0,
            char_cursor: 0,
            prev: EOF,
        }
    }

    pub fn iter(mut self) -> impl Iterator<Item = Token<'a>> {
        std::iter::from_fn(move || self.advance_token()).filter(|token| token.kind != Whitespace)
    }

    pub fn advance_token(&mut self) -> Option<Token<'a>> {
        let initial_byte_cursor = self.byte_cursor;
        let initial_char_cursor = self.char_cursor;

        let first_char = self.bump_char()?;

        let token_kind = match first_char {
            c if c.is_whitespace() => self.whitespace(),
            '_' | 'a'..='z' | 'A'..='Z' => self.ident_or_keyword(initial_byte_cursor),
            digit @ '0'..='9' => self.number(digit),
            '"' => self.string_literal(),
            '\'' => self.char_const(),
            symbol if symbol.is_ascii_punctuation() => self.punct(symbol),
            _ => Error(UnexpectedCharacter),
        };

        Some(Token::new(
            token_kind,
            initial_byte_cursor..self.byte_cursor,
            initial_char_cursor..self.char_cursor,
            self.src,
        ))
    }

    fn whitespace(&mut self) -> TokenKind {
        self.eat_whitespace();
        Whitespace
    }

    fn ident_or_keyword(&mut self, initial_byte_cursor: usize) -> TokenKind {
        self.eat_ident_or_keyword();
        match &self.src[initial_byte_cursor..self.byte_cursor] {
            // Section 6.4.1 Keywords
            "auto" | "break" | "case" | "char" | "const" | "continue" | "default" | "do"
            | "double" | "else" | "enum" | "extern" | "float" | "for" | "goto" | "if"
            | "inline" | "int" | "long" | "register" | "restrict" | "return" | "short"
            | "signed" | "sizeof" | "static" | "struct" | "switch" | "typedef" | "union"
            | "unsigned" | "void" | "volatile" | "while" | "_Bool" | "_Complex" | "_Imaginary" => {
                Keyword
            }
            _ => Ident,
        }
    }

    fn string_literal(&mut self) -> TokenKind {
        while let Some(c) = self.bump_char() {
            match c {
                '"' => return StrLit,
                '\\' if self.peek_char(0) == '"'
                    || self.peek_char(0) == '\\'
                    || self.peek_char(0) == '\n' =>
                {
                    self.bump_char();
                }
                '\n' => return Error(UnterminatedString),
                _ => (),
            }
        }
        Error(UnterminatedString)
    }

    fn char_const(&mut self) -> TokenKind {
        while let Some(c) = self.bump_char() {
            match c {
                '\'' => return Const(Char),
                '\\' if self.peek_char(0) == '\''
                    || self.peek_char(0) == '\\'
                    || self.peek_char(0) == '\n' =>
                {
                    self.bump_char();
                }
                '\n' => return Error(UnterminatedChar),
                _ => (),
            }
        }
        Error(UnterminatedChar)
    }

    fn number(&mut self, first_digit: char) -> TokenKind {
        self.eat_number_constant(first_digit);
        self.eat_number_suffix();
        Const(Number)
    }

    fn punct(&mut self, first_symbol: char) -> TokenKind {
        match first_symbol {
            '(' | ')' | ',' | ';' | '?' | '[' | ']' | '{' | '}' | '~' => Punct,
            '!' | '^' | '/' | '*' => {
                if self.peek_char(0) == '=' {
                    self.bump_char();
                }
                Punct
            }
            '#' | '=' => {
                if self.peek_char(0) == first_symbol {
                    self.bump_char();
                }
                Punct
            }
            '&' | '+' | '|' => {
                if self.peek_char(0) == first_symbol || self.peek_char(0) == '=' {
                    self.bump_char();
                }
                Punct
            }
            '.' => {
                if self.peek_char(0) == '.' && self.peek_char(1) == '.' {
                    self.bump_char();
                    self.bump_char();
                }
                Punct
            }
            '-' => {
                if let '-' | '=' | '>' = self.peek_char(0) {
                    self.bump_char();
                }
                Punct
            }
            '<' => {
                match self.peek_char(0) {
                    '=' | ':' | '%' => {
                        self.bump_char();
                    }
                    '<' if self.peek_char(1) == '=' => {
                        self.bump_char();
                        self.bump_char();
                    }
                    '<' if self.peek_char(1) != '=' => {
                        self.bump_char();
                    }
                    _ => (),
                }
                Punct
            }
            '>' => {
                match self.peek_char(0) {
                    '=' => {
                        self.bump_char();
                    }
                    '>' if self.peek_char(1) == '=' => {
                        self.bump_char();
                        self.bump_char();
                    }
                    '>' if self.peek_char(1) != '=' => {
                        self.bump_char();
                    }
                    _ => (),
                }
                Punct
            }
            ':' => {
                if self.peek_char(0) == '>' {
                    self.bump_char();
                }
                Punct
            }
            '%' => {
                match self.peek_char(0) {
                    '=' | '>' => {
                        self.bump_char();
                    }
                    ':' => {
                        self.bump_char();
                        if self.peek_char(0) == '%' && self.peek_char(1) == ':' {
                            self.bump_char();
                            self.bump_char();
                        }
                    }
                    _ => (),
                }
                Punct
            }
            _ => Error(UnknownPunctuator),
        }
    }

    fn eat_whitespace(&mut self) {
        while self.peek_char(0).is_whitespace() && !self.is_eof() {
            self.bump_char();
        }
    }

    fn eat_ident_or_keyword(&mut self) {
        while let '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' = self.peek_char(0) {
            self.bump_char();
        }
    }

    fn eat_number_constant(&mut self, first_digit: char) {
        if first_digit == '0' {
            match self.peek_char(0) {
                'x' | 'X' => {
                    self.bump_char();
                    self.eat_hexadecimal_digits();
                }
                '0'..='7' => {
                    self.eat_octal_digits();
                }
                '.' => { /* TODO */ }
                _ => return,
            }
        } else {
            self.eat_decimal_digits();
        }
    }

    fn eat_number_suffix(&mut self) {
        /* TODO */
    }

    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        while let '0'..='9' = self.peek_char(0) {
            has_digits = true;
            self.bump_char();
        }
        has_digits
    }

    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        while let '0'..='9' | 'a'..='f' | 'A'..='F' = self.peek_char(0) {
            has_digits = true;
            self.bump_char();
        }
        has_digits
    }

    fn eat_octal_digits(&mut self) -> bool {
        let mut has_digits = false;
        while let '0'..='7' = self.peek_char(0) {
            has_digits = true;
            self.bump_char();
        }
        has_digits
    }

    pub fn peek_char(&self, nth: usize) -> char {
        self.remaining().chars().nth(nth).unwrap_or(EOF)
    }

    pub fn bump_char(&mut self) -> Option<char> {
        let mut chars = self.remaining().chars();

        let c = chars.next()?;
        self.byte_cursor += self.remaining().len() - chars.as_str().len();
        self.char_cursor += 1;
        self.prev = c;
        Some(c)
    }

    pub fn remaining(&self) -> &'a str {
        &self.src[self.byte_cursor..]
    }

    pub fn is_eof(&self) -> bool {
        self.remaining().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eof() {
        let mut lexer = Lexer::new("");
        assert_debug_snapshot!(lexer.advance_token(), "None");
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("\t\r\n int");
        assert_debug_snapshot!(
            lexer.advance_token(),
            r#"
Some(
    [Whitespace@0..4],
)"#
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens: Vec<Token> = Lexer::new("abc _def gh_1").iter().collect();
        assert_debug_snapshot!(
            tokens,
            r#"
[
    [Ident@0..3],
    [Ident@4..8],
    [Ident@9..13],
]"#
        );
    }

    #[test]
    fn test_keywords() {
        let tokens: Vec<Token> = Lexer::new("int float if for").iter().collect();
        assert_debug_snapshot!(
            tokens,
            r#"
[
    [Keyword@0..3],
    [Keyword@4..9],
    [Keyword@10..12],
    [Keyword@13..16],
]"#
        );
    }

    #[test]
    fn test_char_consts() {
        let tokens: Vec<Token> = Lexer::new(r"'a' '\\' '\n'").iter().collect();
        assert_debug_snapshot!(
            tokens,
            r#"
[
    [Const(Char)@0..3],
    [Const(Char)@4..8],
    [Const(Char)@9..13],
]"#
        );
    }
}
