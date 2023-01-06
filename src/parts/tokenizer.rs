use indexmap::IndexMap;

use crate::parts::treecompiler::compile_tree;

use super::datastructures::{Token, StackNode};

pub fn tokenize (file_content:String, file_path:String, cli_args:Vec<String>) -> Box<IndexMap<String, Box<StackNode>>> {//create tokens from the file
	// TODO: Rework tree compiler to work on a single stack with more consistent rules.
	//buffers
	let mut tokenlist:Vec<Token> = vec![];
	let mut charbuff:String = String::new();
	let mut is_string:bool = false;
	let mut last_character:char = '\n';
	let mut list_stack:Vec<Token> = vec![];
	let mut commenting = false;

	let raw_lines = file_content.split("\n");
	//line loop
	for (ln, raw_line) in raw_lines.enumerate() {
		let line = raw_line.clone().trim().to_string();
		//Character loop
		for (cn, character) in line.chars().enumerate() {
			if !commenting
			{
				if is_string {
					if character == '\'' && last_character != '\\' {
						is_string = false;
						tokenlist.push(Token::StringLit(Box::new(charbuff.clone())));
						charbuff = String::new()
					}
					else {
						if last_character == '\\' {
							charbuff.pop();
							match character {
								'n' => {
									charbuff.push('\n');
									//println!("newline");
								}
								'r' => {
									charbuff.push('\r');
								}
								't' => {
									charbuff.push('\t');
								}
								'\\' => {
									charbuff.push('\\');
								}
								_ => {
									charbuff.push(character.clone());
								}
							}
						}
						else {
							charbuff.push(character.clone());
						}
					}
					last_character = character.clone();
					continue;
				}
				match character {
					'=' => {

						// let mut newtok:Box<String> = Box::new(String::new());
						// match tokenlist.last().unwrap() {
						// 	Token::Variable(name) => newtok = Box::new(*name.clone()),
						// 	_ => {}
						// }
						// tokenlist.pop();
						tokenlist.push(Token::Assign);
					}
					'{' => {
						tokenlist.push(Token::StartScope);
					}
					'}' => {
						tokenlist.push(Token::EndScope);
					}
					'#' => {
						commenting = true;
					}
					'(' => {
						match tokenlist.last() {
							Some(val) => {
								if val == &Token::FuncDeff(Box::new(String::new())) {
									tokenlist.pop();
									tokenlist.push(Token::FuncDeff(Box::new(charbuff)));
								}
								else {
									tokenlist.push(Token::FuncCall(Box::new(charbuff)));
								}
							},
							None => tokenlist.push(Token::FuncCall(Box::new(charbuff))),
						}
						charbuff = String::new();
					}
					//delimeters
					' '|'\t'|','|';'|')'|']'|'[' => {
						if charbuff != "" {
							//keywords
							if charbuff == "return" {
								tokenlist.push(Token::Return);
							}
							else if charbuff == "fun" {
								tokenlist.push(Token::FuncDeff(Box::new(String::new())));
							}
							else if charbuff == "if" {
								tokenlist.push(Token::If);
							}
							else if charbuff == "elif" {
								tokenlist.push(Token::ElIf);
							}
							else if charbuff == "else" {
								tokenlist.push(Token::El);
							}
							else if charbuff == "for" {
								tokenlist.push(Token::For);
							}
							else if charbuff == "in" {
								tokenlist.push(Token::In);
							}
							else if charbuff == "while" {
								tokenlist.push(Token::While);
							}
							else if charbuff == "loop" {
								tokenlist.push(Token::Loop);
							}
							else if charbuff == "break" {
								tokenlist.push(Token::Break);
							}
							else if vec!["true","t","yes"].contains(&charbuff.to_lowercase().as_str()) {
								tokenlist.push(Token::BooleanLit(Box::new(true)));
							}
							else if vec!["false","f","no"].contains(&charbuff.to_lowercase().as_str()) {
								tokenlist.push(Token::BooleanLit(Box::new(false)));
							}
							else if charbuff.chars().all(|ch| ch.is_alphanumeric() || ch == '_') && !charbuff.chars().all(char::is_numeric) {
								//keywords
								tokenlist.push(Token::Variable(Box::new(charbuff)));
							}
							else if charbuff.chars().all(|ch| ch.is_numeric() || ch == '.') || (charbuff.chars().all(|ch| ch.is_numeric() || ch == '-' || ch == '.') && charbuff.starts_with('-')) {
								if charbuff.contains('.') {
									tokenlist.push(Token::FloatLit(Box::new(charbuff.parse::<f64>().unwrap())));
								}
								else {
									tokenlist.push(Token::IntLit(Box::new(charbuff.parse::<i128>().unwrap())));
								}
							}
							charbuff = String::new();
						}
						if character == ')' {
							tokenlist.push(Token::StatementEnd);
						}
						if character == ',' {
							tokenlist.push(Token::Delimeter);
						}
						if character == ';' {
							tokenlist.push(Token::EndLine);
						}
						if character == ']' {
							match list_stack.last().unwrap() {
								Token::StartVec => {tokenlist.push(Token::EndVec);},
								Token::IndexStart => {tokenlist.push(Token::IndexEnd);},
								_ => {}
							}
							list_stack.pop();
						}
						if character == '[' {
							match tokenlist.last().unwrap() {
								Token::StringLit(_) | Token::StatementEnd | Token::EndVec | Token::IndexEnd | Token::Variable(_) => {
									tokenlist.push(Token::IndexStart);
									list_stack.push(Token::IndexStart);
								},
								_ => {
									tokenlist.push(Token::StartVec);
									list_stack.push(Token::StartVec);
								}
							}
						}
					}
					'\'' => {
						is_string = !is_string;
					}
					_ => {
						charbuff.push(character);
					}
				}
				last_character = character.clone();
			}
		}
		commenting = false;
	}
	//println!("{:?}", tokenlist);
	compile_tree(tokenlist, file_path, cli_args)[0].clone()
}