pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token {
    toke: TokenType,
    start: usize,
    line: usize,
    length: usize,
}

#[derive(Debug)]
enum TokenType {
    // Single-character tokens
    Pal, Par,  // ( )
    Kel, Ker,  // { }
    Com, Dot, Hep, Lus,
    Mic, Fas, Tar,

    // One or two character tokens
    Zap, ZapTis,
    Tis, TisTis,
    Gar, GarTis,
    Gal, GalTis,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, EOF
}

pub fn compile(source: String) {
    let mut scanner = Scanner::new(source);

    let mut line = 99999;

    loop {
        let token = scanner.scan_token();

        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!("{:02?} {} {}", token.toke, token.length, token.start)
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF)
        };

        let c = self.advance();
        if Self::is_alpha(c) {
            return self.identifier()
        };
        if Self::is_digit(c) {
            return self.number()
        };

        match c {
            // 1 character lexemes
            b'(' => self.make_token(TokenType::Pal),
            b')' => self.make_token(TokenType::Par),
            b'{' => self.make_token(TokenType::Kel),
            b'}' => self.make_token(TokenType::Ker),
            b';' => self.make_token(TokenType::Mic),
            b',' => self.make_token(TokenType::Com),
            b'.' => self.make_token(TokenType::Dot),
            b'-' => self.make_token(TokenType::Hep),
            b'+' => self.make_token(TokenType::Lus),
            b'/' => self.make_token(TokenType::Fas),
            b'*' => self.make_token(TokenType::Tar),

            // 2 character lexemes
            b'!' => if self.mate(b'=') {
                        self.make_token(TokenType::ZapTis)
                    } else {
                        self.make_token(TokenType::Zap)
                    },
            b'=' => if self.mate(b'=') {
                        self.make_token(TokenType::TisTis)
                    } else {
                        self.make_token(TokenType::Tis)
                    },
            b'<' => if self.mate(b'=') {
                        self.make_token(TokenType::GalTis)
                    } else {
                        self.make_token(TokenType::Gal)
                    },
            b'>' => if self.mate(b'=') {
                        self.make_token(TokenType::GarTis)
                    } else {
                        self.make_token(TokenType::Gar)
                    },

            // literal tokens
            b'"' => self.string(),

            _   => self.error_token("huh".to_string()),
        }
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn mate(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false
        } else if self.source.as_bytes()[self.current] != expected {
            return false
        }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.".to_string())
        };

        self.advance();
        self.make_token(TokenType::String)
    }

    fn identifier(&mut self) -> Token {
        while Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.as_bytes()[self.start] {
            b'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            b'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            b'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            b'i' => self.check_keyword(1, 1, "f", TokenType::If),
            b'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            b'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            b'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            b'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            b's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            b'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            b'w' => self.check_keyword(1, 4, "hile", TokenType::While),

            b'f' => if self.current - self.start > 1 {
                return match self.source.as_bytes()[self.start + 1] {
                    b'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                    b'o' => self.check_keyword(2, 1, "r", TokenType::For),
                    b'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            },

            b't' => if self.current - self.start > 1 {
                return match self.source.as_bytes()[self.start + 1] {
                    b'h' => self.check_keyword(2, 2, "is", TokenType::This),
                    b'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                    _ => TokenType::Identifier,
                }
            } else {
                TokenType::Identifier
            },

            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, _length: usize, rest: &str, toke: TokenType) -> TokenType {
        // let comp: String = self.source[self.start + start..self.current]
        //     .iter()
        //     .collect();

        // if comp.as_str() == rest {
        //     return toke;
        // }

        TokenType::Identifier
    }

    fn number(&mut self) -> Token {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == b'.' && Self::is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                b' ' | b'\r' | b'\t' => { self.advance(); break; },

                b'\n' => {
                             self.line += 1;
                             self.advance();
                             break;
                         },
                b'/' => {
                    if self.peek_next() == b'/' {
                        // A comment goes until the end of the line.
                        while self.peek() != b'\n' && !self.is_at_end() {
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

    fn peek(&self) -> u8 {
        self.source.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            return b'\0'
        }
        self.source.as_bytes()[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek() == b'\0'
//        self.source.as_bytes()[self.current] == b'\0'
    }

    fn make_token(&self, toke: TokenType) -> Token {
        Token {
            toke,
            start: self.start,
            line: self.current - self.start,
            length: self.line,
        }
    }

    fn error_token(&self, err: String) -> Token {
        Token {
            toke: TokenType::Error,
            start: 0, //TODO = err???
            line: self.line,
            length: err.len(),
        }
    }

    //TODO probably a rust way to do this
    fn is_digit(c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    //TODO probably a rust way to do this
    fn is_alpha(c: u8) -> bool {
        c >= b'a' && c <= b'z' ||
            c >= b'A' && c <= b'Z' ||
            c == b'_'
    }
}
