use std::cell::RefCell;

use crate::token::*;
use crate::scanner::*;
use crate::vm::InterpretError;
use crate::chunk::*;

pub struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser,
    chunk: &'a mut Chunk,
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current: Token::default(),
            previous: Token::default(),
            had_error: RefCell::new(false),
            panic_mode: RefCell::new(false),
        }
    }
}

impl<'a> Compiler<'a> {
    pub fn new(source: &String, chunk: &'a mut Chunk) -> Self {
        Self {
            scanner: Scanner::new(source),
            parser: Parser::new(),
            chunk,
        }
    }

    pub fn compile(&mut self) -> Result<(), InterpretError> {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression");

        self.end_compiler();

        if *self.parser.had_error.borrow() {
            Err(InterpretError::Compile)
        } else {
            Ok(())
        }
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
        if *self.parser.panic_mode.borrow() {
            return;
        };

        self.parser.panic_mode.replace(true);

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

    fn consume(&mut self, toke: TokenType, msg: &str) {
        if self.parser.current.toke == toke {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return.into());
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }
}
