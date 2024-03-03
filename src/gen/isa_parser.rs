
#![allow(unused)]

use std::{fs::File, io::Read};

use serde::Deserialize;


#[derive(Deserialize)]
pub struct ISA {
    pub min_version: String,
    pub version: String,
    pub isa_information: Vec<IsaInformation>,
    pub prefixes: Vec<Prefix>,
    pub groups: Vec<Group>
}

#[derive(Deserialize)]
pub struct IsaInformation {
    pub description: String,
    pub last_opcode_idx: u8,
    pub last_throw_prefixed_opcode_idx: u8,
    pub last_wide_prefixed_opcode_idx: u8,
    pub last_deprecated_prefixed_opcode_idx: u8,
    pub last_callruntime_prefixed_opcode_idx: u8,
}

#[derive(Deserialize)]
pub struct Prefix {
    pub name: String,
    pub description: String,
    pub opcode_idx: u8,
}

#[derive(Deserialize)]
pub struct Group{
    pub title: String,
    pub description: String,
    pub instructions: Vec<Instruction>,
}


#[derive(Deserialize)]
pub struct Instruction {
    pub sig: String,
    pub acc: String,
    pub opcode_idx: Vec<u8>,
    pub format: Vec<String>,
}

impl ISA {

    pub fn from_json_file(path: &str) -> Self {
        let mut file = File::open(path).expect("Fail to open isa json file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Fail to read content from isa json file");
        ISA::from_json(&content)
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(&json).expect("Failed to parse JSON")
    }
}