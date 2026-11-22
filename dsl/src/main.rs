use std::{env};
use dsl::{run};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: {} <program_file> <function_name> [arg1 arg2 ...]", args[0]);
        std::process::exit(1);
    }

    println!("{:?}", args);
    run(32);
}
