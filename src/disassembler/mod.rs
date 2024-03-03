use std::process::exit;


pub struct Disassembler {
    filename: String,
}

impl Disassembler {
    pub fn new(args: &[String]) -> Self {
        if let Option::Some(filename) = Disassembler::get_filename(args) {
            return Disassembler{
                filename: filename.to_owned()
            };
        }
        exit(0);
    }

    fn get_filename(args: &[String]) -> Option<&String> {
        if args.len() < 1 {
            println!("abc file expected");
            return Option::None;
        }
        Option::Some(&args[0])
    }

    pub fn run(&self) {
        println!("dumpping {} ...", self.filename);
    }
}