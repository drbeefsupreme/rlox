use crate::token::*;
use crate::scanner::*;

pub struct Compiler {
    scanner: Scanner,
}

impl Compiler {
    pub fn new(source: &String) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    pub fn compile(&mut self) {
        //TODO wtf is this
        let mut line = 99999;

        loop {
            let token = self.scanner.scan_token();

            if token.line != line {
                print!("{:04} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }

            //TODO check this
            println!("{:02?} {}", token.toke, token.lexeme);

            if token.toke == TokenType::EOF {
                break;
            }
        }
    }
}
