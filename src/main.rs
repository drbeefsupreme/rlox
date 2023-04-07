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
    chunk.write(OpCode::OpConstant.into());
    chunk.write(constant as u8); // probably the wrong thing to do

    chunk.disassemble("test chunk");
}
