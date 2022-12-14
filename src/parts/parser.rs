use std::{io::{stdin, self}};

use indexmap::IndexMap;

use super::{datastructures::{StackNode, NodeType}, std::standard::{add, sub, mul, div, read, equal, notequal, greater, less, greaterequal, lessequal, filewrite, or, and, in_operator, contains_operator, cat, foreign_function_interface, split, remove_ws, replace}};

pub fn parse_tree(root:Box<StackNode>, file_path:String){
	//Variables
	let mut stack:Box<Vec<Box<IndexMap<String, Box<StackNode>>>>> = Box::new(vec![Box::new(IndexMap::new())]);
	let mut garbage_stack:Box<Vec<Box<Vec<Box<String>>>>> = Box::new(vec![]);
	let mut user_ret = Box::new(StackNode::default());
	user_ret.ntype = Box::new(NodeType::None);
	parse_node(&mut user_ret, &mut Box::new(true), root.clone(), &mut stack, &mut garbage_stack, file_path);
}

fn push_to_stack (current_node:Box<StackNode>, node_to_push:Box<StackNode>, stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, garbage_stack:&mut Vec<Box<Vec<Box<String>>>>, file_path:String) {
	if *current_node.ntype == NodeType::Index {
		// assign to index of variable
		fn rec_get_ind(indexes_vector:Vec<usize>, mut var:&mut StackNode, curr_node: StackNode, node_to_push: Box<StackNode>, args_list: Box<Vec<Box<StackNode>>>, stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Vec<Box<Vec<Box<String>>>>, file_path:String) {
			
			for ind in indexes_vector.iter().rev() {
				var = &mut var.args[*ind];
			}
			*var = *node_to_push;
		}
		fn rec_get_var(curr_node:StackNode, mut indexes_vector:&mut Vec<usize>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Vec<Box<Vec<Box<String>>>>, file_path:String) -> Box<String> {
			if *curr_node.ntype == NodeType::Variable {
				return curr_node.operation.clone();
			}
			else if *curr_node.ntype == NodeType::Index{
				let mut user_ret = Box::new(StackNode::default());
				user_ret.ntype = Box::new(NodeType::None);
				let args_list = parse_node_list(&mut user_ret, false, curr_node.args.clone(), &mut stack, &mut Box::new(garbage_stack.clone()), file_path.clone());
				match *args_list[0].ntype.clone() {
					NodeType::Str(_) => todo!("hashmap implementation"),
					NodeType::Int(val) => {indexes_vector.push(*val as usize)},
					NodeType::Float(val) => {indexes_vector.push(*val as usize)},
					NodeType::Bool(val) => {indexes_vector.push(*val as usize)},
					_ => todo!(),
				}
				return rec_get_var(*curr_node.args[1].clone(), indexes_vector, stack, garbage_stack, file_path);
			}
			Box::new(String::new())
		}
		let mut st_end:usize = stack.len()-1;
		let mut indexes_vector:Vec<usize> = vec![];
		let ind_key = *rec_get_var(*current_node.clone(), &mut indexes_vector, stack, garbage_stack, file_path.clone());
		// let mut stack_ref_counter = Rc::new(RefCell::new(stack));
		// let mut itter_stack = Rc::clone(&stack_ref_counter);
		// let mut stackb1 = Rc::clone(&itter_stack);
		// let mut stackb2 = Rc::clone(&stackb1);
		let mut stackclone = stack.clone();
		for (st_num, mut layer) in stack.iter_mut().enumerate().rev() {
			if layer.contains_key(&ind_key.clone()) {
				let mut user_ret = Box::new(StackNode::default());
				user_ret.ntype = Box::new(NodeType::None);
				let mut args_list = parse_node_list(&mut user_ret, false, current_node.args.clone(), &mut stackclone.clone(), &mut Box::new(garbage_stack.clone()), file_path.clone());
				rec_get_ind(indexes_vector, &mut *layer.get_mut(&ind_key).unwrap(), *current_node.clone(), node_to_push.clone(), args_list, &mut stackclone.clone(), &mut Box::new(garbage_stack.clone()), file_path.clone());
				//println!("layer===={:?}", layer);
				break;
			}
		}
	}
	else {
		let mut st_end:usize = stack.len()-1;
		for (st_num, mut layer) in stack.iter_mut().enumerate().rev() {
			if layer.contains_key(&*current_node.operation.clone()) {
				*layer.get_mut(&*current_node.operation).unwrap() = node_to_push.clone();
				return;
			}
		}
		stack[st_end].insert(*current_node.operation.clone(), node_to_push.clone());
	}
}

fn parse_node_list(mut user_return:&mut Box<StackNode>, is_scope:bool, mut node_list:Box<Vec<Box<StackNode>>>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Box<Vec<Box<Vec<Box<String>>>>>, file_path:String) -> Box<Vec<Box<StackNode>>> {
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
		let new_node = parse_node(&mut user_return, &mut executing, Box::new(*curr_node.clone()), &mut stack, &mut garbage_stack, file_path.clone());
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

fn get_variable(key:String, stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>) -> Box<StackNode> {
	for layer in stack.iter().rev() {
		if layer.contains_key(&key) {
			return layer.get(&key).unwrap().clone()
		}
	}
	return Box::new(StackNode::default());
}

pub fn parse_node(mut user_return: &mut Box<StackNode>, mut executing:&mut Box<bool>, node:Box<StackNode>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>, mut garbage_stack:&mut Box<Vec<Box<Vec<Box<String>>>>>, file_path:String) -> Box<StackNode>{
	//println!("\n{:?}", node);
	if !vec![NodeType::Condition, NodeType::Def].contains(&node.ntype) {
		**executing = true;
	}
	let mut args_list:Box<Vec<Box<StackNode>>> = Box::new(vec![]);
	let mut ret_node:Box<StackNode> = Box::new(StackNode::default());
	*ret_node.ntype = NodeType::None;
	if **executing {
		if !vec![NodeType::Def, NodeType::Loop].contains(&*node.ntype.clone()) {
			args_list = parse_node_list(&mut user_return, false, node.args.clone(), &mut stack, &mut garbage_stack, file_path.clone());
		}
		match *node.ntype {
			NodeType::Call => {
				match node.operation.as_str() {
					"" => {
						parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path);
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
					"push" => {
						for layer in stack.iter_mut().rev() {
							if layer.contains_key(node.args[0].operation.clone().as_str()) {
								layer.get_mut(&*node.args[0].operation).unwrap().args.push(args_list[1].clone());
							}
						}
					}
					"pop" => {
						for layer in stack.iter_mut().rev() {
							if layer.contains_key(node.args[0].operation.clone().as_str()) {
								layer.get_mut(&*node.args[0].operation).unwrap().args.pop();
							}
						}
					}
					"FFI" => {
						ret_node.ntype = foreign_function_interface(args_list);
					}
					"split" => {
						ret_node = split(args_list);
					}
					"trim" => {
						ret_node.ntype = remove_ws(args_list);
					}
					"replace" => {
						ret_node.ntype = replace(args_list);
					}
					"cat" => {
						ret_node.ntype = cat(args_list);
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
					"and" => {
						ret_node.ntype = and(args_list[0].ntype.clone(), args_list[1].ntype.clone());
					}
					"or" => {
						ret_node.ntype = or(args_list[0].ntype.clone(), args_list[1].ntype.clone());
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
						ret_node.ntype = equal(args_list[0].clone(), args_list[1].clone());
					}
					"NE" => {
						//NOT EQUAL
						ret_node.ntype = notequal(args_list[0].clone(), args_list[1].clone());
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
						ret_node.ntype = read(args_list[0].ntype.clone(), file_path);
					}
					"write" => {
						//write to files
						//arg1 is the file name, arg3 is writemode (append or truncate or nothing for default(the default is truncate))
						//arg2 is the content to write to the file.
						//eg: write("folder/filepath.txt", "FulcrumRS is a cool language.", "a"|"t"|None);
						ret_node.ntype = filewrite(args_list[0].ntype.clone(), args_list[1].ntype.clone(), args_list[2].ntype.clone(), file_path);
					}
					"input" => {
						let mut ret_val = String::new();
						io::Write:: flush(&mut io::stdout()).expect("flush failed!");
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
						let result = parse_node_list(&mut passable_return, true, user_func.scope, &mut stack, &mut garbage_stack, file_path);
						// for trash in garbage_stack.last().unwrap().iter() {
						// 	stack.remove(&**trash);
						// }
						stack.pop();
						return passable_return;
					}
				}
			},
			NodeType::Def => {
				push_to_stack(node.clone(), node.clone(), &mut stack, &mut garbage_stack, file_path);
				*ret_node = StackNode { operation: node.operation, ntype: node.ntype, args: node.args, scope: node.scope };
			},
			NodeType::Assign => {
				//println!("LHS +++ {:?}", node.args[0]);
				push_to_stack(node.args[0].clone(), args_list[1].clone(), &mut stack, &mut garbage_stack, file_path);
			},
			NodeType::Variable => {
				ret_node = get_variable(*node.operation, &mut stack).clone();
			},
			NodeType::Return => {
				//*user_return.operation = String::from("return");
				if args_list.len() > 0 {
					*user_return.ntype = *args_list[0].ntype.clone();
				}
				else {
					*user_return.ntype = NodeType::Return;
				}
				
			},
			NodeType::Condition => {
				match node.operation.as_str() {
					"if" => {
						match &*args_list[0].ntype {
							NodeType::Str(val) => {
								if **val != "" {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Int(val) => {
								if **val > 0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Float(val) => {
								if **val > 0.0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Bool(val) => {
								if **val {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							_ => {},
						}
						*ret_node.ntype = NodeType::Bool(Box::new(false));
					}
					"elif" => {
						//println!("{:?}", node);
						match &*args_list[0].ntype {
							NodeType::Str(val) => {
								if **val != "" {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Int(val) => {
								if **val > 0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Float(val) => {
								if **val > 0.0 {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							NodeType::Bool(val) => {
								if **val {
									let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
									**executing = false;
									*ret_node.ntype = NodeType::Bool(Box::new(true));
								}
							},
							_ => {},
						}
						*ret_node.ntype = NodeType::Bool(Box::new(false))
					}
					"else" => {
						let mut scope_list:Box<Vec<Box<StackNode>>> = parse_node_list(&mut user_return, true, node.scope, &mut stack, &mut garbage_stack, file_path);
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
								return parse_node(&mut user_return, &mut executing, args_list[1].args[val.parse::<usize>().expect("The provided string caused an invalid cast to integer.")].clone(), &mut stack, &mut garbage_stack, file_path)
							},
							NodeType::Int(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack, file_path)
							},
							NodeType::Float(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack, file_path)
							},
							NodeType::Bool(val) => {
								return parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone(), &mut stack, &mut garbage_stack, file_path)
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
			NodeType::Loop => {
				match node.operation.as_str() {
					"for" => {
						stack.push(Box::new(IndexMap::new()));
						let alist = parse_node_list(&mut user_return, false, node.args.clone(), &mut stack, &mut garbage_stack, file_path.clone());
						match *alist[1].ntype.clone() {
							NodeType::Str(val) => {
								for value in val.chars() {
									let mut val_for_stack = Box::new(StackNode::default());
									*val_for_stack.ntype = NodeType::Str(Box::new(String::from(value)));
									push_to_stack(node.args[0].clone(), val_for_stack.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									if *user_return.ntype == NodeType::Break {
										*user_return.ntype = NodeType::None;
										break;
									}
									if *user_return.ntype != NodeType::None {
										break;
									}
								}
							},
							NodeType::Int(val) => {
								for value in 0..*val {
									let mut val_for_stack = Box::new(StackNode::default());
									*val_for_stack.ntype = NodeType::Int(Box::new(value));
									push_to_stack(node.args[0].clone(), val_for_stack.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									if *user_return.ntype == NodeType::Break {
										*user_return.ntype = NodeType::None;
										break;
									}
									if *user_return.ntype != NodeType::None {
										break;
									}
								}
							},
							NodeType::Vector => {
								for value in alist[1].args.iter() {
									push_to_stack(node.args[0].clone(), value.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									if *user_return.ntype == NodeType::Break {
										*user_return.ntype = NodeType::None;
										break;
									}
									if *user_return.ntype != NodeType::None {
										break;
									}
								}
							},
							_ => {
								println!("ERROR<loop::for>(make an issue on the git repo with this message and your source code): {:?}", stack);
							},
						}
						stack.pop();
						//**executing = false;
					}
					"while" => {
						loop {
							let alist = parse_node_list(&mut user_return, false, node.args.clone(), &mut stack, &mut garbage_stack, file_path.clone());
							match *alist[0].ntype.clone() {
								NodeType::Bool(val) => {
									if *val {
										parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path.clone());
									}
									else {
										break;
									}
								}
								_ => {
	
								}
							}
							if *user_return.ntype == NodeType::Break {
								*user_return.ntype = NodeType::None;
								break;
							}
							if *user_return.ntype != NodeType::None {
								break;
							}
						}
						//**executing = false;
					}
					"loop" => {
						loop {
							parse_node_list(&mut user_return, true, node.scope.clone(), &mut stack, &mut garbage_stack, file_path.clone());
							if *user_return.ntype == NodeType::Break {
								*user_return.ntype = NodeType::None;
								break;
							}
							if *user_return.ntype != NodeType::None {
								break;
							}
						}
						//**executing = false;
					}
					_ => {

					}
				}
			}
			NodeType::Operator => {
				match node.operation.as_str() {
					"contains" => {
						ret_node.ntype = contains_operator(&mut args_list[0].clone(), args_list[1].clone());
					}
					"in" => {

					}
					_ => {

					}
				}
			}
			NodeType::Break => {
				*user_return.ntype = NodeType::Break;
			}
		}
	}
	ret_node
}