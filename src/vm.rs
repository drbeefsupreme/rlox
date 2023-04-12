use crate::chunk::*;
use crate::value::*;
use crate::STACK_MAX;
use crate::compiler::*;

pub struct VM {
//    chunk: Chunk,
    ip: usize,  // instruction index
    stack: Vec<Value>,
//    stack_top: usize,
}

pub enum InterpretError {
    Compile,
    Runtime,
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl VM {
    pub fn new() -> VM {
        VM {
//            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::<Value>::with_capacity(STACK_MAX),
//            stack_top: 0,
        }
    }

    // might be unnecessary
    pub fn free(&mut self) {}

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().expect("nothing left to pop off stack")
    }

    pub fn interpret(&mut self, source: &String) -> Result<(), InterpretError> {
        let mut compiler = Compiler::new(source);
        compiler.compile();
        Ok(())
    }

    fn reset_stack(&mut self) {
        self.stack = Vec::<Value>::with_capacity(STACK_MAX);
//        self.stack_top = 0;
    }

    fn run(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
        loop {
            if cfg!(debug_assertions) {

                print!("         ");
                for val in self.stack.iter() {
                    print!("[ {:?} ]", val);
                }
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte(chunk);
            match instruction {
                OpCode::Return => {
                    println!("{:?}", self.pop());
                    return Ok(());
                },
                OpCode::Constant => {
                    let constant: Value = self.read_constant(chunk);
                    self.push(constant);
                },
                OpCode::Negate => {
                    if let Value::Number(f) = self.pop() {
                        self.push(Value::Number(-f));
                    }
                },
                OpCode::Add => self.binary_op(BinaryOp::Add),
                OpCode::Sub => self.binary_op(BinaryOp::Sub),
                OpCode::Mul => self.binary_op(BinaryOp::Mul),
                OpCode::Div => self.binary_op(BinaryOp::Div),
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let op: OpCode = chunk.read_code(self.ip).into();
        self.ip += 1;
        op
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let val: Value = chunk.read_constant(chunk.read_code(self.ip) as usize);
        self.ip += 1;
        val
    }

    fn binary_op(&mut self, op: BinaryOp) {
        // might need a while loop here, 15.2
        if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
            match op {
                BinaryOp::Add => self.push(Value::Number(a + b)),
                BinaryOp::Sub => self.push(Value::Number(a - b)),
                BinaryOp::Mul => self.push(Value::Number(a * b)),
                BinaryOp::Div => self.push(Value::Number(a / b)),
            };
        }
    }
}
