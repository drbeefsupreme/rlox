#![allow(dead_code)]

mod chunk;
mod value;

use std::env::args;
use chunk::*;
use value::*;

fn main() {
    let _args: Vec<String> = args().collect();

    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(Value::Number(1.2));
    chunk.write(OpCode::OpConstant.into(), 123);
    chunk.write(constant as u8, 123); // probably the wrong thing to do
    chunk.write(OpCode::OpReturn.into(), 123);

    chunk.disassemble("test chunk");
}
