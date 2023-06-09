use std::collections::HashMap;

use crate::chunk::*;
use crate::value::*;
use crate::STACK_MAX;
use crate::compiler::*;

pub struct VM {
//    chunk: Chunk,
    ip: usize,  // instruction index
    stack: Vec<Value>,
//    stack_top: usize,
    globals: HashMap<String, Value>,
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
    Less,
    Greater,
}

impl VM {
    pub fn new() -> Self {
        Self {
//            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
//            stack_top: 0,
            globals: HashMap::new(),
        }
    }

    // might be unnecessary
    pub fn free(&mut self) {
        //vm.strings?
        self.globals = HashMap::new();
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("nothing left to pop off stack")
    }

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - distance - 1]
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(source, &mut chunk);
        compiler.compile()?;

        self.ip = 0;
        let result = self.run(&chunk);
        chunk.free();

        result
    }

    fn reset_stack(&mut self) {
        self.stack = Vec::<Value>::with_capacity(STACK_MAX);
//        self.stack_top = 0;
    }

    fn runtime_error<T: ToString> (
        &mut self,
        chunk: &Chunk,
        msg: &T,
    ) -> Result<(), InterpretError> {
        let line = chunk.get_line(self.ip - 1);
        eprintln!("{}", msg.to_string());
        eprintln!("[line {line}] in script");
        self.reset_stack();

        Err(InterpretError::Runtime)
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
                    //println!("{:?}", self.pop());
                    // Exit interpreter.
                    return Ok(());
                },
                OpCode::Constant => {
                    // ended up cloning here after peppering & around the call stack leading to this,
                    // not sure if its the right choice? I think it might be, though
                    let constant: Value = self.read_constant(chunk).clone();
                    self.push(constant);
                },
                OpCode::Negate => {
                    if !self.peek(0).is_number() {
                        return self.runtime_error(chunk, &"Operand must be a number.");
                    }

                    let value = self.pop();
                    self.push(-value);
                },
                OpCode::Add   => self.binary_op(BinaryOp::Add),
                OpCode::Sub   => self.binary_op(BinaryOp::Sub),
                OpCode::Mul   => self.binary_op(BinaryOp::Mul),
                OpCode::Div   => self.binary_op(BinaryOp::Div),
                OpCode::Nil   => self.push(Value::Nil),
                OpCode::True  => self.push(Value::Bool(true)),
                OpCode::False => self.push(Value::Bool(false)),
                OpCode::Pop   => { self.pop(); }, // why do I need to put this in a block?
                OpCode::GetGlobal => {
                    let name = self.read_constant(chunk).clone();

                    if let Value::Str(s) = name {
                        match self.globals.get(&s) {
                            //TODO do i clone here?
                            Some(v) => self.push(v.clone()),
                            None    => return self.runtime_error(chunk, &format!("Undefined variable {s}")),
                        }
                    }
                }
                OpCode::DefineGlobal => {
                    let name = self.read_constant(chunk).clone();
                    if let Value::Str(s) = name {
                        // Might not be necessary here, but we pop the value after we add it to
                        // the hash table to ensure that the VM can still find the value if a GC
                        // is triggerd in the middle of adding it to the hash table since it requires
                        // dynamic allocation when it resizes.
                        self.globals.insert(s, self.peek(0).clone());
                        self.pop();
                    } else {
                        return self.runtime_error(chunk, &"Unable to read constant from table.");
                    }
                }
                OpCode::SetGlobal => {
                    let name = self.read_constant(chunk).clone();
                    if let Value::Str(s) = name {
                        if !self.globals.contains_key(&s) {
                            return self.runtime_error(chunk, &format!("Undefined variable {s}"));
                        } else {
                            self.globals.insert(s, self.peek(0).clone());
                        }
                    } else {
                        return self.runtime_error(chunk, &"Unable to read constant from table.");
                    }
                }
                OpCode::GetLocal => {
                    let slot = self.read_byte(chunk) as usize;
                    self.stack[slot] = self.peek(0).clone();
                }
                OpCode::SetLocal => {
                    let slot = self.read_byte(chunk) as usize;
                    self.stack[slot] = self.peek(0).clone();
                }
                OpCode::Equal => {
                    let (b, a) = (self.pop(), self.pop());
                    self.push(Value::Bool(b == a));
                },
                OpCode::Not   => {
                    let value = self.pop();
                    self.push(Value::Bool(value.is_falsey()));
                },
                OpCode::Greater => self.binary_op(BinaryOp::Greater),
                OpCode::Less    => self.binary_op(BinaryOp::Less),
                OpCode::Print   => {
                    let value = self.pop();
                    println!("{}\n", value);
                }
            }
        }
    }

    fn read_byte(&mut self, chunk: &Chunk) -> OpCode {
        let op: OpCode = chunk.read_code(self.ip).into();
        self.ip += 1;
        op
    }

    fn read_constant<'a>(&'a mut self, chunk: &'a Chunk) -> &Value {
        let val: &Value = chunk.read_constant(chunk.read_code(self.ip) as usize);
        self.ip += 1;
        val
    }

    fn binary_op(&mut self, op: BinaryOp) {
        let (b, a) = (self.pop(), self.pop());
        match op {
            BinaryOp::Add => self.push(a + b),
            BinaryOp::Sub => self.push(a - b),
            BinaryOp::Mul => self.push(a * b),
            BinaryOp::Div => self.push(a / b),
            BinaryOp::Less => self.push(Value::Bool(a < b)),
            BinaryOp::Greater => self.push(Value::Bool(a > b)),
        }
        // might need a while loop here, 15.2
        // if let (Value::Number(b), Value::Number(a)) = (self.pop(), self.pop()) {
        //     match op {
        //         BinaryOp::Add => self.push(Value::Number(a + b)),
        //         BinaryOp::Sub => self.push(Value::Number(a - b)),
        //         BinaryOp::Mul => self.push(Value::Number(a * b)),
        //         BinaryOp::Div => self.push(Value::Number(a / b)),
        //     };
        // }
    }
}
