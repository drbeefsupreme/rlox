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
    locals: RefCell<Vec<Local>>, // this should probably be an array
    scope_depth: usize,
}

pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

struct Local {
    name: Token,
    depth: Option<usize>,
}

impl Local {
    pub fn new(name: Token, depth: Option<usize>) -> Self {
        Self {
            name,
            depth,
        }
    }
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
    prefix: Option<fn(&mut Compiler, bool)>,
    infix: Option<fn(&mut Compiler, bool)>,
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
    pub fn new(source: &str, chunk: &'a mut Chunk) -> Self {
        Self {
            scanner: Scanner::new(source),
            parser: Parser::new(),
            chunk,
            rules: Self::build_parse_rule_table(),
            locals: RefCell::new(Vec::new()),
            scope_depth: 0,
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
            // if its already panicking, don't bother accumulating more errors
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

    fn block(&mut self) {
        while !self.check(TokenType::Ker) && !self.check(TokenType::EOF) {
            self.declaration();
        }

        self.consume(TokenType::Ker, "Expect '}' after block.");
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.mate(TokenType::Tis) {
            self.expression();
        } else {
            // Desugars `var a;` into `var a = nil;`
            self.emit_byte(OpCode::Nil.into());
        }

        self.consume(TokenType::Mic,
                     "Expect ';' after variable declaration.");
        self.define_variable(global);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Mic, "Expect ';' after expression.");
        self.emit_byte(OpCode::Pop.into());
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Mic, "Expect ';' after value.");
        self.emit_byte(OpCode::Print.into());
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode.replace(false);

        while self.parser.current.toke != TokenType::EOF {
            if self.parser.previous.toke == TokenType::Mic {
                return;
            }
            use crate::token::TokenType::*;
            match self.parser.current.toke {
                Class | Fun | Var | For
                    | If | While | Print
                    | Return => return,
                _ => self.advance(), //TODO double check this
            }
        }
    }

    fn declaration(&mut self) {
        if self.mate(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if *self.parser.panic_mode.borrow() {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.mate(TokenType::Print) {
            self.print_statement();
        } else if self.mate(TokenType::Kel) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
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

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;

        while !self.locals.borrow().is_empty()
            && self.locals.borrow().last().unwrap().depth.unwrap() > self.scope_depth
            {
                self.emit_byte(OpCode::Pop.into());
                self.locals.borrow_mut().pop();
            }
    }

    fn binary(&mut self, _: bool) {
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

    fn literal(&mut self, _: bool) {
        let toke = self.parser.previous.toke;
        match toke {
            TokenType::False => self.emit_byte(OpCode::False.into()),
            TokenType::Nil   => self.emit_byte(OpCode::Nil.into()),
            TokenType::True  => self.emit_byte(OpCode::True.into()),
            _                => panic!("Invalid literal"),
        }
    }

    fn grouping(&mut self, _: bool) {
        self.expression();
        self.consume(TokenType::Par, "Expect ')' after expression.");
    }

    fn number(&mut self, _: bool) {
        //TODO clone lexeme?
        let value = Value::Number(self.parser.previous.lexeme.parse().unwrap());
        self.emit_constant(value);
    }

    fn string(&mut self, _: bool) {
        let len = self.parser.previous.lexeme.len() - 1;
        let value = Value::Str(self.parser.previous.lexeme[1..len].to_string());
        self.emit_constant(value);
    }

    fn named_variable(&mut self, name: String, can_assign: bool) {
        //TODO double check the expect message
//        let arg = self.identifier_constant(name).expect("No corresponding variable.");

        let (arg, get_op, set_op) = if let Some(local_arg) = self.resolve_local(&name) {
            (local_arg, OpCode::GetLocal, OpCode::SetLocal)
        } else {
            (
                self.identifier_constant(name).unwrap() as u8,
                OpCode::GetGlobal,
                OpCode::SetGlobal,
            )
        };

        if can_assign && self.mate(TokenType::Tis) {
            self.expression();
            self.emit_bytes(set_op.into(), arg);
        } else {
            self.emit_bytes(get_op.into(), arg);
        }
    }

    fn resolve_local(&mut self, name: &String) -> Option<u8> {
        for (e, v) in self.locals.borrow().iter().rev().enumerate() {
            if v.name.lexeme == *name {
                if v.depth.is_none() {
                    self.error("Can't read local variable in its own initalizer.");
                }
                return Some((self.locals.borrow().len() - e - 1) as u8)
            }
        }
        None
    }

    fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous.lexeme.clone(), can_assign);
    }

    fn unary(&mut self, _: bool) {
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
            let can_assign: bool = precedence <= Precedence::Assignment;
            prefix_rule(self, can_assign);

            while precedence <= self.rules[self.parser.current.toke.int_value()].precedence {
                self.advance();

                if let Some(infix_rule) = self.rules[self.parser.previous.toke.int_value()].infix {
                    infix_rule(self, can_assign);
                }

                if can_assign && self.mate(TokenType::Tis) {
                    self.error("Invalid assignment target.");
                }
            }
        } else {
            self.error("Expect expression.");
        }
    }

    fn identifier_constant(&mut self, lex: String) -> Option<u8> {
        self.make_constant(Value::Str(lex))
    }

    fn add_local(&mut self, name: Token) {
        let local = Local::new(name, None);
        self.locals.borrow_mut().push(local);
    }

    fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        } else {
            let name = self.parser.previous.lexeme.clone();
            if self
                .locals
                .borrow()
                .iter()
                .filter(|x| x.name.lexeme == name)
                .count()
                != 0 {
                self.error("already a variable with this name in this scope.");
            } else {
                self.add_local(self.parser.previous.clone());
            }
        }
    }

    fn parse_variable(&mut self, msg: &str) -> u8 {
        //TODO the error should probably be threaded through differently - i think i'm
        // mixing up C and Rust conventions here
        self.consume(TokenType::Identifier, msg);

        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }

        //TODO do i clone here?
        self.identifier_constant(self.parser.previous.lexeme.clone()).expect(msg)
    }

    fn mark_initialized(&mut self) {
        let last = self.locals.borrow().len() - 1;
        let mut locals = self.locals.borrow_mut();
        locals[last].depth = Some(self.scope_depth);
    }

    fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_bytes(OpCode::DefineGlobal.into(), global);
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
                prefix: Some(|c, b| c.grouping(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Hep.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.unary(b)),
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Term,
            };
        rules[TokenType::Lus.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Term,
            };
        rules[TokenType::Fas.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Factor,
            };
        rules[TokenType::Tar.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Factor,
            };
        rules[TokenType::Number.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.number(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::False.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.literal(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::True.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.literal(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Nil.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.literal(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Zap.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.unary(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::ZapTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Equality,
            };
        rules[TokenType::TisTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Equality,
            };
        rules[TokenType::Gar.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::GarTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::Gal.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::GalTis.int_value()] =
            ParseRule {
                prefix: None,
                infix: Some(|c, b| c.binary(b)),
                precedence: Precedence::Comparison,
            };
        rules[TokenType::String.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.string(b)),
                infix: None,
                precedence: Precedence::None,
            };
        rules[TokenType::Identifier.int_value()] =
            ParseRule {
                prefix: Some(|c, b| c.variable(b)),
                infix: None,
                precedence: Precedence::None,
            };

        rules
    }

}
