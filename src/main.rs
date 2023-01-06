mod parts;
use std::env;
use std::fs;

use parts::tokenizer::tokenize;
const VERSION:&str = "0.8.9";
fn main() {
    let args:Vec<String> = env::args().collect();
	if args.len() == 0 {
		//No args
		println!("__Fulcrum_interpreter_v{VERSION}__\n USAGE: fulcrum <filepath.ful>");
	}
	else if args[1].starts_with("-"){
		match args[1].to_lowercase().as_str() {
			"-c" => {
				tokenize(args[2].replace("\"", "\\\"").clone(), args[0].clone(), args);
			}
			"-v" => {
				print!("{VERSION}");
			}
			_ => {
				println!("INVALID FLAG");
			}
		}
	}
	else {
		let contents = fs::read_to_string(args[1].clone())
			.expect("Invalid filepath");
		tokenize(contents.clone(), args[1].clone(), args);
	}
}
