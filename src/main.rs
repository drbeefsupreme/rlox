mod chunk;
mod memory;

use std::env::args;
use chunk::*;


fn main() {
    let args: Vec<String> = args().collect();

    let mut chunk = Chunk::new();

    chunk.write_opcode(OpCode::OpReturn);

    println!("{:?}", chunk.disassemble());
}
