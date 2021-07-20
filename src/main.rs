use std::fs;

use clap::{load_yaml, App};

use crate::parser::Parser;

mod opcode;
mod parser;
mod token;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let file = fs::read(input_file)
        .expect(format!("File with name {} does not exist", input_file).as_str());

    let parser = Parser::new(&file);

    let tokens = parser.parse();

    tokens.iter().for_each(|token| println!("{}", token));
}
