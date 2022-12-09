mod parts;
use std::env;
use std::fs;

use parts::tokenizer::tokenize;

fn main() {
    let args:Vec<String> = env::args().collect();
	let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");
	tokenize(contents.clone(), args[1].clone());
}
