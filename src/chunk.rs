use crate::value::*;

#[derive(Debug)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            _ => unimplemented!("Invalid OpCode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::<u8>::new(),
            constants: Vec::<Value>::new(),
            lines: Vec::<usize>::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn free(&mut self) {
        self.code = Vec::<u8>::new();
        self.constants = Vec::<Value>::new();
    }

    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("  | ");
        } else {
            print!("{:?} ", self.lines[offset]);
        }

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
            OpCode::OpConstant => self.const_instruction("OP_CONSTANT", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn const_instruction(&self, name: &str, offset: usize) -> usize {
        // index of constant in self.constants
        let constant = self.code[offset + 1];
        print!("{name}     {} ", constant);
        self.constants[constant as usize].print();
        offset + 2
    }
}
