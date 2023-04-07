#![allow(dead_code)]

mod chunk;
mod value;

use std::env::args;
use chunk::*;

fn main() {
    let _args: Vec<String> = args().collect();

    let mut chunk = Chunk::new();

    chunk.write_opcode(OpCode::OpReturn);

    chunk.disassemble("test chunk");
}
