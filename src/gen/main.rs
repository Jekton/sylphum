use std::time::Instant;

use isa_parser::ISA;
use common::collections::ForEach;

mod isa_parser;


fn main() {
    // transfered from arkcompiler/isa/isa.yaml since a JSON is
    // more commonly known.
    let isa = ISA::from_json_file("src/gen/isa.json");
    println!("isa version = {}", isa.version);
    isa.groups.for_each(| group | {
        group.instructions.for_each(| inst | {
            println!("instruction {}", inst.sig);
        });
    })
}
