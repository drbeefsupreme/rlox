use std::cell::RefCell;

use crate::token::*;
use crate::scanner::*;
use crate::vm::InterpretError;
use crate::chunk::*;

pub struct Compiler {
    scanner: Scanner,
    parser: Parser,
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_error: RefCell::new(false),
        }
    }
}

impl Compiler {
    pub fn new(source: &String) -> Self {
        Self {
            scanner: Scanner::new(source),
            parser: Parser::new(),
        }
    }

    // pub fn compile(&mut self) -> Result<Chunk, InterpretError> {
    //     let mut line = 0;

    //     loop {
    //         let token = self.scanner.scan_token();

    //         if token.line != line {
    //             print!("{:04} ", token.line);
    //             line = token.line;
    //         } else {
    //             print!("   | ");
    //         }

    //         //TODO check this
    //         println!("{:02?} '{}'", token.toke, token.lexeme);

    //         if token.toke == TokenType::EOF {
    //             break;
    //         }
    //     }

    //     Ok(Chunk::new())
    // }

    pub fn compile(&mut self) -> Result<Chunk, InterpretError> {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression");
        Ok(Chunk::new())
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.toke != TokenType::Error {
                break;
            }

            self.error_at_current(&self.parser.current.lexeme);
        }
    }

    fn error_at_current(&self, msg: &str) {
        self.error_at(&self.parser.current, msg);
    }

    fn error_at(&self, token: &Token, msg: &str) {
        eprint!("[line {}] Error", token.line);

        if token.toke == TokenType::EOF {
            eprint!(" at end");
        } else if token.toke == TokenType::Error {
            // Nothing
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {msg}");
        self.parser.had_error.replace(true);
    }

    fn error(&self, msg: &str) {
        self.error_at(&self.parser.previous, msg);
    }

    fn expression(&self) {}

    fn consume(&self, toke: TokenType, msg: &str) {}
}
