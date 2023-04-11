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

    let mut constant = chunk.write_constant(Value::Number(1.2));
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant as u8, 123); // probably the wrong thing to do

    constant = chunk.write_constant(Value::Number(3.4));
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Add.into(), 123);

    constant = chunk.write_constant(Value::Number(5.6));
    chunk.write(OpCode::Constant.into(), 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::Div.into(), 123);
    chunk.write(OpCode::Negate.into(), 123);
    chunk.write(OpCode::Return.into(), 123);

    chunk.disassemble("test chunk");

    vm.interpret(&chunk);
    vm.free();
}
