#![allow(dead_code)]

mod chunk;
mod value;
mod vm;
mod compiler;
mod token;
mod scanner;

use std::env;
use std::env::args;
use std::io;
use vm::*;

use text_io::*;

const STACK_MAX: usize = 256;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = args().collect();
    let mut vm: VM = VM::new();

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(&mut vm, &args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }

    vm.free();
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");

        let line = read!("{}\n");
        vm.interpret(&line);
    }
}

fn run_file(vm: &mut VM, path: &str) -> io::Result<InterpretResult> {
    let source = std::fs::read_to_string(path.to_string())?;
    let result = vm.interpret(&source);

    match result {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => (),
    };
    Ok(result)
}
