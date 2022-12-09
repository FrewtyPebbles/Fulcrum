use std::io::{stdin, Read, self};

use indexmap::IndexMap;

use super::{datastructures::{StackNode, NodeType}, std::standard::{add, sub, mul, div, read, equal, notequal, greater, less, greaterequal, lessequal}};

pub fn parse_tree(mut root:Box<StackNode>){
	//Variables
	let mut stack:Box<Vec<Box<IndexMap<String, Box<StackNode>>>>> = Box::new(vec![Box::new(IndexMap::new())]);
	let mut garbage_stack:Box<Vec<Box<Vec<Box<String>>>>> = Box::new(vec![]);
	let mut user_ret = Box::new(StackNode::default());
	user_ret.ntype = Box::new(NodeType::None);
	user_ret.ntype = Box::new(NodeType::None);
	parse_node(&mut user_ret, &mut Box::new(true), root.clone(), &mut stack, &mut garbage_stack);
}

fn push_to_stack (mut current_node:Box<StackNode>, mut node_to_push:Box<StackNode>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Vec<Box<Vec<Box<String>>>>) {
	let mut st_end:usize = stack.len()-1;
	for (st_num, mut layer) in stack.iter_mut().enumerate().rev() {
		if layer.contains_key(&*current_node.operation.clone()) {
			*layer.get_mut(&*current_node.operation).unwrap() = node_to_push.clone();
			return;
		}
	}
	stack[st_end].insert(*current_node.operation.clone(), node_to_push.clone());
}

fn parse_node_list(mut user_return:&mut Box<StackNode>, is_scope:bool, mut node_list:Box<Vec<Box<StackNode>>>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Box<Vec<Box<Vec<Box<String>>>>>) -> Box<Vec<Box<StackNode>>> {
	let mut ret_list:Box<Vec<Box<StackNode>>> = Box::new(vec![]);
	//boolean is passed in per layer of the stack to enable or disable execution based on
	//conditional statements
	let mut executing:Box<bool> = Box::new(true);
	
	if is_scope {
		stack.push(Box::new(IndexMap::new()));
		// garbage_stack.push(Box::new(vec![]));
	}
	for curr_node in node_list.iter() {
		if *user_return.ntype != NodeType::None {
			break;
		}
		let new_node = parse_node(&mut user_return, &mut executing, Box::new(*curr_node.clone()), &mut stack, &mut garbage_stack);
		// if is_scope && *user_return.operation == "return" {
		// 	user_return.ntype = new_node.ntype.clone()
		// }
		ret_list.push(new_node);
	}
	if is_scope {
		// for trash in garbage_stack.last().unwrap().iter() {
		// 	stack.last().unwrap().remove(&**trash);
		// }
		stack.pop();
		// garbage_stack.pop();
	}
	ret_list
}

fn get_variable(mut key:String, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>) -> Box<StackNode> {
	for layer in stack.iter().rev() {
		if layer.contains_key(&key) {
			return layer.get(&key).unwrap().clone()
		}
	}
	return Box::new(StackNode::default())
}

pub fn parse_node(mut user_return: &mut Box<StackNode>, mut executing:&mut Box<bool>, mut node:Box<StackNode>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Box<Vec<Box<Vec<Box<String>>>>>) -> Box<StackNode>{
	//println!("\n{:?}", node);
	if !vec![NodeType::Condition, NodeType::Def].contains(&node.ntype) {
		**executing = true;
	}
	let mut args_list:Box<Vec<Box<StackNode>>> = Box::new(vec![]);
	let mut ret_node:Box<StackNode> = Box::new(StackNode::default());
	*ret_node.ntype = NodeType::None;
	if **executing {
		if *node.ntype != NodeType::Def {
			args_list = parse_node_list(&mut user_return, false, node.args.clone(), &mut stack, &mut garbage_stack);
		}
		match *node.ntype {
			NodeType::Call => {
				match node.operation.as_str() {
					"" => {
						let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack);
						//global scope
					}
					"print" => {
						match &*args_list[0].ntype {
							NodeType::Str(val) => print!("{val}"),
							NodeType::Int(val) => print!("{val}"),
							NodeType::Float(val) => print!("{val}"),
							NodeType::Bool(val) => print!("{val}"),
							_ => {},
						}
					}
					//put the math and complex functions into std/standard.rs as functions
					"add" => {
						ret_node.ntype = add(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"sub" => {
						ret_node.ntype = sub(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"mul" => {
						ret_node.ntype = mul(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"div" => {
						ret_node.ntype = div(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"N" => {
						//NOT
						ret_node.ntype = match *args_list[0].ntype.clone() {
							NodeType::Str(val) => if *val == "" {Box::new(NodeType::Bool(Box::new(true)))} else {Box::new(NodeType::Bool(Box::new(false)))},
							NodeType::Int(val) => Box::new(NodeType::Bool(Box::new(*val == 0))),
							NodeType::Float(val) => Box::new(NodeType::Bool(Box::new(*val == 0.0))),
							NodeType::Bool(val) => Box::new(NodeType::Bool(Box::new(!*val))),
							_ => {Box::new(NodeType::Bool(Box::new(true)))},
						}
					}
					"E" => {
						//EQUAL
						ret_node.ntype = equal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"NE" => {
						//NOT EQUAL
						ret_node.ntype = notequal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"G" => {
						//GREATER
						ret_node.ntype = greater(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"L" => {
						//LESS
						ret_node.ntype = less(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"GE" => {
						//GREATER OR EQUAL
						ret_node.ntype = greaterequal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"LE" => {
						//LESS OR EQUAL
						ret_node.ntype = lessequal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"read" => {
						//read from files and return contents
						//eg: read("folder/filepath.txt");
						ret_node.ntype = read(args_list[0].ntype.clone());
					}
					"write" => {
						//write to files
						//arg1 is the file name, arg3 is writemode (append or truncate or nothing for default(the default is truncate))
						//arg2 is the content to write to the file.
						//eg: write("folder/filepath.txt", "FulcrumRS is a cool language.", "a"|"t"|None);
						match &*args_list[0].ntype {
							NodeType::Str(val) => print!("{val}"),
							NodeType::Int(val) => print!("{val}"),
							NodeType::Float(val) => print!("{val}"),
							NodeType::Bool(val) => print!("{val}"),
							_ => {},
						}
					}
					"input" => {
						let mut ret_val = String::new();
						io::Write::flush(&mut io::stdout()).expect("flush failed!");
						stdin().read_line(&mut ret_val).unwrap();
						*ret_node.ntype = NodeType::Str(Box::new(String::from(String::from(ret_val.strip_suffix("\n").unwrap()).strip_suffix("\r").unwrap())))
					}
					"INT" => {
						match &*args_list[0].ntype {
							NodeType::Str(val) => {
								//println!("val:{val};");
								*ret_node.ntype = NodeType::Int(Box::new(val.parse::<i128>().unwrap()))
							},
							NodeType::Int(val) => *ret_node.ntype = NodeType::Int(Box::new(**val)),
							NodeType::Float(val) => *ret_node.ntype = NodeType::Int(Box::new(**val as i128)),
							NodeType::Bool(val) => *ret_node.ntype = NodeType::Int(if **val {Box::new(1)} else {Box::new(0)}),
							_ => {},
						}
					}
					"FLOAT" => {
						match &*args_list[0].ntype {
							NodeType::Str(val) => *ret_node.ntype = NodeType::Float(Box::new(val.parse::<f64>().unwrap())),
							NodeType::Int(val) => *ret_node.ntype = NodeType::Float(Box::new(**val as f64)),
							NodeType::Float(val) => *ret_node.ntype = NodeType::Float(Box::new(**val)),
							NodeType::Bool(val) => *ret_node.ntype = NodeType::Float(if **val {Box::new(1.0)} else {Box::new(0.0)}),
							_ => {},
						}
					}
					"STRING" => {
						match &*args_list[0].ntype.clone() {
							NodeType::Str(val) => *ret_node.ntype = NodeType::Int(Box::new(val.parse::<i128>().unwrap())),
							NodeType::Int(val) => *ret_node.ntype = NodeType::Int(Box::new(**val)),
							NodeType::Float(val) => *ret_node.ntype = NodeType::Int(Box::new(**val as i128)),
							NodeType::Bool(val) => *ret_node.ntype = NodeType::Int(if **val {Box::new(1)} else {Box::new(0)}),
							_ => {},
						}
					}
					"BOOL" => {
						match &*args_list[0].ntype.clone() {
							NodeType::Str(val) => *ret_node.ntype = NodeType::Bool(if vec!["true","t","yes"].contains(&&*val.as_str().to_lowercase()) {Box::new(true)} else {Box::new(false)}),
							NodeType::Int(val) => *ret_node.ntype = NodeType::Bool(if **val > 0 {Box::new(true)} else {Box::new(false)}),
							NodeType::Float(val) => *ret_node.ntype = NodeType::Bool(if **val > 0.0 {Box::new(true)} else {Box::new(false)}),
							NodeType::Bool(val) => *ret_node.ntype = NodeType::Bool(if **val {Box::new(true)} else {Box::new(false)}),
							_ => {},
						}
					}
					_ => {
						let mut user_func = get_variable(*node.operation, &mut stack);
						stack.push(Box::new(IndexMap::new()));
						for (an, arg) in args_list.iter().enumerate() {
							let mut name = user_func.args[an].operation.clone();
							user_func.args[an] = arg.clone();
							user_func.args[an].operation = name;
							let st_end = stack.len() -1;
							stack[st_end].insert(*user_func.args[an].operation.clone(), arg.clone());
							//push_to_stack(user_func.args[an].clone(), arg.clone(), &mut stack, &mut garbage_stack);
						}
						let mut passable_return = Box::new(StackNode::default());
						passable_return.ntype = Box::new(NodeType::None);
						let result = parse_node_list(&mut passable_return, true, user_func.scope, &mut stack, &mut garbage_stack);
						// for trash in garbage_stack.last().unwrap().iter() {
						// 	stack.remove(&**trash);
						// }
						stack.pop();
						return passable_return;
					}
				}
			},
			NodeType::Def => {
				push_to_stack(node.clone(), node.clone(), &mut stack, &mut garbage_stack);
				*ret_node = StackNode { operation: node.operation, ntype: node.ntype, args: node.args, scope: node.scope };
			},
			NodeType::Assign => {
				push_to_stack(node, args_list[0].clone(), &mut stack, &mut garbage_stack);
				// if !stack.contains_key(&*node.operation) {
				// 	let gs_len = garbage_stack.len()-1;
				// 	garbage_stack[gs_len].push(node.operation.clone());
				// 	stack.insert(*node.operation, args_list[0].clone());
				// }
				// else {
				// 	*stack.get_mut(&*node.operation).unwrap() = Box::new(*(*args_list)[0].clone());
				// }
			},
			NodeType::Variable => {
				ret_node = get_variable(*node.operation, &mut stack);
			},
			NodeType::Return => {
				//*user_return.operation = String::from("return");
				*user_return.ntype = *args_list[0].ntype.clone();
			},
			NodeType::Condition => {
				match node.operation.as_str() {
					"if" => {
						match &*args_list[0].ntype {
							NodeType::Str(val) => {
								if **val != "" {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Int(val) => {
								if **val > 0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Float(val) => {
								if **val > 0.0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Bool(val) => {
								if **val {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							_ => {},
						}
						*ret_node.ntype = NodeType::Bool(Box::new(false));
					}
					"elif" => {
						println!("{:?}", node);
						match &*args_list[0].ntype {
							NodeType::Str(val) => {
								if **val != "" {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Int(val) => {
								if **val > 0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Float(val) => {
								if **val > 0.0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Bool(val) => {
								if **val {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							_ => {},
						}
						*ret_node.ntype = NodeType::Bool(Box::new(false))
					}
					"else" => {
						let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack);
						*ret_node.ntype = NodeType::Bool(Box::new(false))
					}
					_ => {
						*ret_node.ntype = NodeType::None
					}
				}
			},
			NodeType::Vector => {
				*ret_node.ntype = NodeType::Vector;
				ret_node.args = node.args;
			},
			NodeType::Index => {
				//println!("{:?}", stack);
				let arg1 = *args_list[1].ntype.clone();
				let arg2 = *args_list[0].ntype.clone();
				match arg1 {
					NodeType::Str(val) => {
						match arg2 {
							NodeType::Str(val2) => {
								*ret_node.ntype = NodeType::Str(Box::new(String::from(val.chars().collect::<Vec<char>>()[val2.parse::<usize>().expect("The provided string caused an invalid cast to integer.")])));
							},
							NodeType::Int(val2) => {
								*ret_node.ntype = NodeType::Str(Box::new(String::from(val.chars().collect::<Vec<char>>()[*val2 as usize])));
							},
							NodeType::Float(val2) => {
								*ret_node.ntype = NodeType::Str(Box::new(String::from(val.chars().collect::<Vec<char>>()[*val2 as usize])));
							},
							NodeType::Bool(val2) => {
								*ret_node.ntype = NodeType::Str(Box::new(String::from(val.chars().collect::<Vec<char>>()[*val2 as usize])));
							},
							_ => {},
						}
					}
					_ => {
						match arg2 {
							NodeType::Str(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[val.parse::<usize>().expect("The provided string caused an invalid cast to integer.")].clone(), &mut stack, &mut garbage_stack)
							},
							NodeType::Int(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack)
							},
							NodeType::Float(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack)
							},
							NodeType::Bool(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack)
							},
							_ => {},
						}
					}
				}
			},
			NodeType::Str(val) => *ret_node.ntype = NodeType::Str(val),
			NodeType::Int(val) => *ret_node.ntype = NodeType::Int(val),
			NodeType::Float(val) => *ret_node.ntype = NodeType::Float(val),
			NodeType::Bool(val) => *ret_node.ntype = NodeType::Bool(val),
			NodeType::None => todo!(),
		}
	}
	ret_node
}