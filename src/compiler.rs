pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token {
    toke: TokenType,
    lexeme: String,
    line: usize,
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

pub fn compile(source: &String) {
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

        //TODO check this
        println!("{:02?} {}", token.toke, token.lexeme);
    }
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
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

        match c {
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

            // 2 character lexemes
            '!' => if self.mate('=') {
                        self.make_token(TokenType::ZapTis)
                    } else {
                        self.make_token(TokenType::Zap)
                    },
            '=' => if self.mate('=') {
                        self.make_token(TokenType::TisTis)
                    } else {
                        self.make_token(TokenType::Tis)
                    },
            '<' => if self.mate('=') {
                        self.make_token(TokenType::GalTis)
                    } else {
                        self.make_token(TokenType::Gal)
                    },
            '>' => if self.mate('=') {
                        self.make_token(TokenType::GarTis)
                    } else {
                        self.make_token(TokenType::Gar)
                    },

            // literal tokens
            '"' => self.string(),

            '0'..='9' => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            _   => self.error_token("unrecognized character"),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn mate(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        } else if self.source[self.current] != expected {
            return false
        }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.")
        };

        self.advance();
        self.make_token(TokenType::String)
    }

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
        if self.peek() == '.' && self.peek_next().is_numeric() {
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
                    if self.peek_next() == '/' {
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
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek() == '\0'
//        self.source.as_bytes()[self.current] == b'\0'
    }

    fn make_token(&self, toke: TokenType) -> Token {
        Token {
            toke,
            lexeme: self.source[self.start..self.current].iter().collect(),
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
