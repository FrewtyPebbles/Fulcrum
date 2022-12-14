mod parts;
use std::env;
use std::fs;

use parts::tokenizer::tokenize;
const VERSION:&str = "0.6.3";
fn main() {
    let args:Vec<String> = env::args().collect();
	if args.len() == 1 {
		//No args
		println!("__Fulcrum_interpreter_v{VERSION}__\n USAGE: fulcrum <filepath.ful>");
	}
	else {
		let contents = fs::read_to_string(args[1].clone())
			.expect("Should have been able to read the file");
		tokenize(contents.clone(), args[1].clone());
	}
}
