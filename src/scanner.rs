use crate::token::*;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'a> {
    pub source: &'a String,
    pub char_indices: Peekable<CharIndices<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            char_indices: source.clone().char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some((pos, c)) = self.char_indices.next() {
            tokens.push(self.scan_token(pos, c));
        }

        tokens
    }

    pub fn scan_token(&mut self, pos: usize, c: char) -> Token {
        match c {
//            None => self.make_token(TokenType::EOF),
            // 1 character lexemes
            '(' => self.make_token(TokenType::Pal),
            ')' => self.make_token(TokenType::Par),
            '{' => self.make_token(TokenType::Kel),
            '}' => self.make_token(TokenType::Ker),
            ';' => self.make_token(TokenType::Mic),
            ',' => self.make_token(TokenType::Com),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Hep),
            '+' => self.make_token(TokenType::Lus),
            '/' => self.make_token(TokenType::Fas),
            '*' => self.make_token(TokenType::Tar),

            //  Digraphs
            '!' => {
                match self.char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_tis) => self.make_token(TokenType::ZapTis),
                    None => self.make_token(TokenType::Zap),
                }
            },
            '=' => {
                match self.char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_tis) => self.make_token(TokenType::TisTis),
                    None => self.make_token(TokenType::Tis),
                }
            },
            '>' => {
                match self.char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_tis) => self.make_token(TokenType::GarTis),
                    None => self.make_token(TokenType::Gar),
                }
            },
            '<' => {
                match self.char_indices.next_if_eq(&(pos + 1, '=')) {
                    Some(_tis) => self.make_token(TokenType::GalTis),
                    None => self.make_token(TokenType::Gal),
                }
            },

            _ if c.is_numeric() => {
                let mut num = c.to_string();
                num.push(self.char_indices
                    .by_ref()
                    .take_while(|(_pos, d)| {
                        d.is_numeric()
                    })
                    .map(|(_pos, d)| d)
                    .collect() as char);
                if self.char_indices.peek() == Some('.') {
                    self.char_indices.next(); // consumes .
                    num.push(self.char_indices
                        .by_ref()
                        .take_while(|(_pos, d)| {
                            d.is_numeric()
                        })
                        .map(|(_pos, d)| d)
                        .collect() as char);
                }
                self.make_number_token(num)
            }

            //  Literals
            '"' => self.string(),
            _ if c.is_numeric() => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),

            _ => self.error_token("unrecognized character"),
        }
    }

    // pub fn scan_token(&mut self, c: char) -> Token {
    //     self.skip_whitespace();
    //     self.start = self.current;

    //     // if self.is_at_end() {
    //     //     return self.make_token(TokenType::EOF)
    //     // };

    //     let c = self.advance();

    //     match c {
    //         None => self.make_token(TokenType::EOF),
    //         // 1 character lexemes
    //         Some('(') => self.make_token(TokenType::Pal),
    //         Some(')') => self.make_token(TokenType::Par),
    //         Some('{') => self.make_token(TokenType::Kel),
    //         Some('}') => self.make_token(TokenType::Ker),
    //         Some(';') => self.make_token(TokenType::Mic),
    //         Some(',') => self.make_token(TokenType::Com),
    //         Some('.') => self.make_token(TokenType::Dot),
    //         Some('-') => self.make_token(TokenType::Hep),
    //         Some('+') => self.make_token(TokenType::Lus),
    //         Some('/') => self.make_token(TokenType::Fas),
    //         Some('*') => self.make_token(TokenType::Tar),

    //         // 2 character lexemes
    //         Some('!') => if self.mate('=') {
    //                     self.make_token(TokenType::ZapTis)
    //                 } else {
    //                     self.make_token(TokenType::Zap)
    //                 },
    //         Some('=') => if self.mate('=') {
    //                     self.make_token(TokenType::TisTis)
    //                 } else {
    //                     self.make_token(TokenType::Tis)
    //                 },
    //         Some('<') => if self.mate('=') {
    //                     self.make_token(TokenType::GalTis)
    //                 } else {
    //                     self.make_token(TokenType::Gal)
    //                 },
    //         Some('>') => if self.mate('=') {
    //                     self.make_token(TokenType::GarTis)
    //                 } else {
    //                     self.make_token(TokenType::Gar)
    //                 },

    //         // literal tokens
    //         Some('"') => self.string(),
    //         _ if c.unwrap().is_numeric() => self.number(),
    //         _ if c.unwrap().is_alphabetic() || c.unwrap() == '_' => self.identifier(),

    //         _ => self.error_token("unrecognized character"),
    //     }
    // }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    // fn mate(&mut self, expected: char) -> bool {
    //     if self.is_at_end() {
    //         return false
    //     } else if self.source[self.current] != expected {
    //         return false
    //     }

    //     self.current += 1;
    //     true
    // }

    fn string(&mut self) -> Token {
        let mut last_matched: char = '\0';

        let s: String = self.char_indices
            .by_ref()
            .take_while(|(_pos, c)| {
                last_matched = *c;
                *c != '"'
            })
            .map(|(_pos, c)| { c })
            .collect();

        match last_matched {
            '"' => self.make_string_token(s),
            _   => self.error_token("Unterminated literal.")
        }
    }

    // fn string(&mut self) -> Token {
    //     while self.peek() != '"' && !self.is_at_end() {
    //         if self.peek() == '\n' {
    //             self.line += 1;
    //         }
    //         self.advance();
    //     }

    //     if self.is_at_end() {
    //         return self.error_token("Unterminated string.")
    //     };

    //     self.advance();
    //     self.make_token(TokenType::String)
    // }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),

            'f' => if self.current - self.start > 1 {
                return match self.source[self.start + 1] {
                    'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                    'o' => self.check_keyword(2, 1, "r", TokenType::For),
                    'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            },

            't' => if self.current - self.start > 1 {
                return match self.source[self.start + 1] {
                    'h' => self.check_keyword(2, 2, "is", TokenType::This),
                    'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            },

            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self,
                     start: usize,
                     _length: usize,
                     rest: &str,
                     toke: TokenType
    ) -> TokenType {
        let comp: String = self.source[self.start + start..self.current]
            .iter()
            .collect();

        if comp.as_str() == rest {
            return toke;
        }

        TokenType::Identifier
    }

    fn number(&mut self) -> Token {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().unwrap().is_numeric() {
            // Consume the "."
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); break; },

                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                },
                '/' => {
                    if self.peek_next().unwrap() == '/' {
                        // A comment goes until the end of the line.
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    } // another break here?
                }

                _ => break,
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
//        self.peek() == '\0'
//        self.source.as_bytes()[self.current] == b'\0'
    }

    fn make_token(&self, toke: TokenType) -> Token {
        Token {
            toke,
            lexeme: self.source[self.start..self.current].iter().collect(),
            line: self.line,
        }
    }

    fn make_string_token(&self, string: String) -> Token {
        Token {
            toke: TokenType::StringLiteral(string),
            line: self.line,
        }
    }

    fn make_number_token(&self, string: String) -> Token {
        Token {
            toke: TokenType::Number(string.parse::<f64>()),
            line: self.line,
        }
    }

    fn error_token(&self, err: &str) -> Token {
        Token {
            toke: TokenType::Error,
            lexeme: err.to_string(),
            line: self.line,
        }
    }
}
