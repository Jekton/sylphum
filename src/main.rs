use std::{env::args, process::exit};

mod disassembler;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        help();
    }
    match args[1].as_str() {
        "disa" | "disassembler" => {
            println!("disassembler");
            let disa = disassembler::Disassembler::new(&args[2..]);
            disa.run();
        }
        _ => {
            println!("Known command {}", args[1]);
            help();
        }
    }
    println!("Hello, world!");
}

fn help() -> ! {
    println!("Usage:");
    println!("  cargo run [disa|disassembler] xxx.abc");
    exit(0);
}
