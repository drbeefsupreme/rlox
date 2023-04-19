use std::cell::RefCell;

use crate::token::*;
use crate::scanner::*;
use crate::vm::InterpretError;
use crate::chunk::*;
use crate::value::*;
use int_enum::IntEnum;

pub struct Compiler<'a> {
    scanner: Scanner,
    parser: Parser,
    chunk: &'a mut Chunk,
    rules: Vec<ParseRule>,
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

#[repr(usize)]
#[derive(PartialEq, PartialOrd, Copy, Clone, IntEnum)]
enum Precedence {
    None       = 0,
    Assignment = 1, // =
    Or         = 2, // or
    And        = 3, // and
    Equality   = 4, // == !=
    Comparison = 5, // < > <= >=
    Term       = 6, // + -
    Factor     = 7, // * /
    Unary      = 8, // ! -
    Call       = 9, // . ()
    Primary    = 10,
}

impl Precedence {
    fn next(self) -> Self {
        if self == Precedence::Primary {
            panic!("no next() after Primary");
        }
        Self::from_int(self.int_value() + 1).unwrap()
    }
}

#[derive(Copy, Clone)]
struct ParseRule {
    prefix: Option<fn(&mut Compiler)>,
    infix: Option<fn(&mut Compiler)>,
    precedence: Precedence,
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
            rules: Self::build_parse_rule_table(),
        }
    }

    pub fn compile(&mut self) -> Result<(), InterpretError> {
        self.advance();
        // self.expression();
        // self.consume(TokenType::EOF, "Expect end of expression");

        while !self.mate(TokenType::EOF) {
            self.declaration();
        }

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

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Mic, "Expect ';' after value.");
        self.emit_byte(OpCode::Print.into());
    }

    fn declaration(&mut self) {
        self.statement();
    }

    fn statement(&mut self) {
        if self.mate(TokenType::Print) {
            self.print_statement();
        }
    }

    fn consume(&mut self, toke: TokenType, msg: &str) {
        if self.parser.current.toke == toke {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn check(&self, toke: TokenType) -> bool {
        self.parser.current.toke == toke
    }

    fn mate(&mut self, toke: TokenType) -> bool {
        if !self.check(toke) {
            return false;
        }
        self.advance();
        true
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

    fn make_constant(&mut self, value: Value) -> Option<u8> {
        let constant = self.chunk.write_constant(value);
        if constant > std::u8::MAX as usize {
            eprintln!("Too many constants in one chunk.");
            return None;
        };

        Some(constant as u8)
    }

    fn emit_constant(&mut self, value: Value) {
        let con = self.make_constant(value);
        match con {
            Some(c) => self.emit_bytes(OpCode::Constant.into(), c),
            None => panic!("emit_constant failed"), // is panic the right thing here?
        }
    }

    fn end_compiler(&mut self) {
        if cfg!(debug_assertions) && !*self.parser.had_error.borrow() {
            self.chunk.disassemble("code");
        }

        self.emit_return();
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.toke;
        let rule = self.get_rule(operator_type).unwrap().precedence.next();

        self.parse_precedence(rule);

        match operator_type {
            TokenType::ZapTis => self.emit_bytes(OpCode::Equal.into(), OpCode::Not.into()),
            TokenType::TisTis => self.emit_byte(OpCode::Equal.into()),
            TokenType::Gar    => self.emit_byte(OpCode::Greater.into()),
            TokenType::GarTis => self.emit_bytes(OpCode::Less.into(), OpCode::Not.into()),
            TokenType::Gal    => self.emit_byte(OpCode::Less.into()),
            TokenType::GalTis => self.emit_bytes(OpCode::Greater.into(), OpCode::Not.into()),

            TokenType::Lus => self.emit_byte(OpCode::Add.into()),
            TokenType::Hep => self.emit_byte(OpCode::Sub.into()),
            TokenType::Tar => self.emit_byte(OpCode::Mul.into()),
            TokenType::Fas => self.emit_byte(OpCode::Div.into()),

           _ => return,
        }
    }

    fn literal(&mut self) {
        let toke = self.parser.previous.toke;
        match toke {
            TokenType::False => self.emit_byte(OpCode::False.into()),
            TokenType::Nil   => self.emit_byte(OpCode::Nil.into()),
            TokenType::True  => self.emit_byte(OpCode::True.into()),
            _                => panic!("Invalid literal"),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::Par, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        //TODO clone lexeme?
        let value = Value::Number(self.parser.previous.lexeme.parse().unwrap());
        self.emit_constant(value);
    }

    fn string(&mut self) {
        let len = self.parser.previous.lexeme.len() - 1;
        let value = Value::Str(self.parser.previous.lexeme[1..len].to_string());
        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.toke;

        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction
        match operator_type {
            TokenType::Hep => self.emit_byte(OpCode::Negate.into()),
            TokenType::Zap => self.emit_byte(OpCode::Not.into()),
            _ => return,
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(prefix_rule) = self.rules[self.parser.previous.toke.int_value()].prefix {
            prefix_rule(self);

            while precedence <= self.rules[self.parser.current.toke.int_value()].precedence {
                self.advance();

                if let Some(infix_rule) = self.rules[self.parser.previous.toke.int_value()].infix {
                    infix_rule(self);
                }
            }
        } else {
            self.error("Expect expression.");
        }
    }

    fn get_rule(&self, toke: TokenType) -> Option<ParseRule> {
        if self.rules.get(toke.int_value()).is_none() {
            None
        } else {
            Some(self.rules[toke.int_value()])
        }
    }

    fn build_parse_rule_table() -> Vec<ParseRule> {
        let mut rules: Vec<ParseRule> = vec!
            [ ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            }; TokenType::NumberOfTokens.int_value()];

        rules[TokenType::Pal.int_value()] =
            ParseRule {
                prefix: Some(|c| c.grouping()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Hep.int_value()] =
            ParseRule {
                prefix: Some(|c| c.unary()),
                infix: Some(|c| c.binary()),
                precedence: Precedence::Term,
            };
        rules[TokenType::Lus.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Term,
            };
        rules[TokenType::Fas.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Factor,
            };
        rules[TokenType::Tar.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Factor,
            };
        rules[TokenType::Number.int_value()] =
            ParseRule {
                prefix: Some(|c| c.number()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::False.int_value()] =
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::True.int_value()] =
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Nil.int_value()] =
            ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Zap.int_value()] =
            ParseRule {
                prefix: Some(|c| c.unary()),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::ZapTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Equality,
            };
        rules[TokenType::TisTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Equality,
            };
        rules[TokenType::Gar.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::GarTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::Gal.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::GalTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::String.int_value()] =
            ParseRule {
                prefix: Some(|c| c.string()),
                infix: None,
                precedence: Precedence::None,
            };

        rules
    }

}
