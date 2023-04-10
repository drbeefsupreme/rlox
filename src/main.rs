#![allow(dead_code)]

mod chunk;
mod value;
mod vm;

use std::env;
use std::env::args;
use chunk::*;
use value::*;
use vm::*;

const STACK_MAX: usize = 256;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let _args: Vec<String> = args().collect();
    let mut vm: VM = VM::new();

    let mut chunk = Chunk::new();

    let constant = chunk.write_constant(Value::Number(1.2));
    chunk.write(OpCode::OpConstant.into(), 123);
    chunk.write(constant as u8, 123); // probably the wrong thing to do
    chunk.write(OpCode::OpNegate.into(), 123);
    chunk.write(OpCode::OpReturn.into(), 123);

    chunk.disassemble("test chunk");

    vm.interpret(&chunk);
    vm.free();
}
