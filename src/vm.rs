use crate::chunk::*;
use crate::value::*;
use crate::STACK_MAX;

pub struct VM {
//    chunk: Chunk,
    ip: usize,  // instruction index
    stack: Vec<Value>,
//    stack_top: usize,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
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

    fn reset_stack(&mut self) {
        self.stack = Vec::<Value>::with_capacity(STACK_MAX);
//        self.stack_top = 0;
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> InterpretResult {
        // self.chunk = chunk;
        self.ip = 0;
        self.run(chunk)
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
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
                OpCode::OpReturn => {
                    println!("{:?}", self.pop());
                    return InterpretResult::Ok;
                },
                OpCode::OpConstant => {
                    let constant: Value = self.read_constant(chunk);
                    self.push(constant);
                },
                OpCode::OpNegate => {
                    if let Value::Number(f) = self.pop() {
                        self.push(Value::Number(-f));
                    }
                },
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let val: OpCode = chunk.read_code(self.ip).into();
        self.ip += 1;
        val
    }

    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let val = chunk.read_constant(chunk.read_code(self.ip) as usize);
        self.ip += 1;
        val
    }
}
