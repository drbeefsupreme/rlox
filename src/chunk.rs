#[derive(Debug)]
pub enum OpCode {
    OpReturn = 0,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::OpReturn,
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
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::<u8>::new(),
        }
    }

    pub fn write_opcode(&mut self, byte: OpCode) {
        self.code.push(byte.into());
    }

    pub fn free(&mut self) {
        self.code = Vec::<u8>::new();
    }

    pub fn disassemble(&self) -> Vec<OpCode> {
        self.code.clone()
                 .into_iter()
                 .map(|v| v.into())
                 .collect()
    }
}
