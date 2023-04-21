use crate::value::*;

#[derive(Debug)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    Nil = 2,
    True = 3,
    False = 4,
    Pop = 5,
    GetGlobal = 6,
    DefineGlobal = 7,
    Equal = 8,
    Greater = 9,
    Less = 10,
    Negate = 11,
    Print = 12,
    Add = 13,
    Sub = 14,
    Mul = 15,
    Div = 16,
    Not = 17,
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    constants: ValueArray,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::<u8>::new(),
            constants: ValueArray::new(),
            lines: Vec::<usize>::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn read_code(&self, ip: usize) -> u8 {
        self.code[ip]
    }

    pub fn read_constant(&self, i: usize) -> &Value {
        self.constants.read_value(i)
    }

    pub fn write_constant(&mut self, value: Value) -> usize {
        self.constants.write(value)
    }

    pub fn get_line(&self, ip: usize) -> usize {
        self.lines[ip]
    }

    pub fn free(&mut self) {
        // might be unnecessary
        self.code = Vec::<u8>::new();
        self.constants = ValueArray::new();
    }

    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("== end test ==");
    }


    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::Return   => self.simple_instruction("OP_RETURN", offset),
            OpCode::Negate   => self.simple_instruction("OP_NEGATE", offset),
            OpCode::Add      => self.simple_instruction("OP_ADD", offset),
            OpCode::Sub      => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::Mul      => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::Div      => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::Nil      => self.simple_instruction("OP_NIL", offset),
            OpCode::True     => self.simple_instruction("OP_TRUE", offset),
            OpCode::False    => self.simple_instruction("OP_FALSE", offset),
            OpCode::Not      => self.simple_instruction("OP_NOT", offset),
            OpCode::Pop      => self.simple_instruction("OP_POP", offset),
            OpCode::Equal    => self.simple_instruction("OP_EQUAL", offset),
            OpCode::Greater  => self.simple_instruction("OP_GREATER", offset),
            OpCode::Less     => self.simple_instruction("OP_LESS", offset),
            OpCode::Print    => self.simple_instruction("OP_PRINT", offset),

            OpCode::Constant => self.const_instruction("OP_CONSTANT", offset),
            OpCode::GetGlobal => self.const_instruction("OP_GET_GLOBAL", offset),
            OpCode::DefineGlobal => self.const_instruction("OP_DEFINE_GLOBAL", offset),
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
        self.constants.print_value(constant as usize);
//        self.constants[constant as usize].print();
        println!("");
        offset + 2
    }
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Nil,
            3 => OpCode::True,
            4 => OpCode::False,
            5 => OpCode::Pop,
            6 => OpCode::GetGlobal,
            7 => OpCode::DefineGlobal,
            8 => OpCode::Equal,
            9 => OpCode::Greater,
            10 => OpCode::Less,
            11 => OpCode::Negate,
            12 => OpCode::Print,
            13 => OpCode::Add,
            14 => OpCode::Sub,
            15 => OpCode::Mul,
            16 => OpCode::Div,
            17 => OpCode::Not,
            _ => unimplemented!("Invalid OpCode"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        code as u8
    }
}
