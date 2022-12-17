use std::{io::{stdin, self}};

use indexmap::IndexMap;

use super::{datastructures::{StackNode, NodeType}};
pub mod standard;

pub struct Parser {
	pub file_path:String,
	pub cli_args:Vec<String>,
	pub stack:Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>
}

impl Parser {
	pub fn parse_tree(&mut self, root:Box<StackNode>){
		//Variables
		let mut user_ret = Box::new(StackNode::default());
		user_ret.ntype = Box::new(NodeType::None);
		self.parse_node(&mut user_ret, &mut Box::new(true), root.clone());
	}
	//used in the function below them
		fn rec_get_ind(&mut self, indexes_vector:Vec<usize>, st_num:usize, ind_key:String, node_to_push: Box<StackNode>) {
			//println!("called3");
			let mut var = &mut *self.stack[st_num].get_mut(&ind_key).unwrap();
			for ind in indexes_vector.iter().rev() {
				var = &mut var.args[*ind];
			}
			*var = node_to_push.clone();
		}

		fn rec_get_var(&mut self, curr_node:StackNode, indexes_vector:&mut Vec<usize>) -> Box<String> {
			if *curr_node.ntype == NodeType::Variable {
				return curr_node.operation.clone();
			}
			else if *curr_node.ntype == NodeType::Index{
				let mut user_ret = Box::new(StackNode::default());
				user_ret.ntype = Box::new(NodeType::None);
				let args_list = self.parse_node_list(&mut user_ret, false, curr_node.args.clone(), false);
				match *args_list[0].ntype.clone() {
					NodeType::Str(_) => todo!("hashmap implementation"),
					NodeType::Int(val) => {indexes_vector.push(*val as usize)},
					NodeType::Float(val) => {indexes_vector.push(*val as usize)},
					NodeType::Bool(val) => {indexes_vector.push(*val as usize)},
					_ => todo!(),
				}
				return self.rec_get_var(*curr_node.args[1].clone(), indexes_vector);
			}
			Box::new(String::new())
		}
	// end of used in the function below them
	fn push_to_stack (&mut self, current_node:Box<StackNode>, node_to_push:Box<StackNode>) {
		if *current_node.ntype == NodeType::Index {
			// assign to index of variable
			//println!("called, {:?}, {:?}", self.stack, (0..self.stack.len()).rev().collect::<Vec<usize>>());
			let mut indexes_vector:Vec<usize> = vec![];
			let ind_key = *self.rec_get_var(*current_node.clone(), &mut indexes_vector);
			// let mut stack_ref_counter = Rc::new(RefCell::new(stack));
			// let mut itter_stack = Rc::clone(&stack_ref_counter);
			// let mut stackb1 = Rc::clone(&itter_stack);
			// let mut stackb2 = Rc::clone(&stackb1);
			for st_num in (0..self.stack.len()).rev() {
				if self.stack[st_num].contains_key(&ind_key.clone()) {
					let mut user_ret = Box::new(StackNode::default());
					user_ret.ntype = Box::new(NodeType::None);
					//let args_list = self.parse_node_list(&mut user_ret, false, current_node.args.clone(), false);
					//println!("called2");
					self.rec_get_ind(indexes_vector, st_num, ind_key, node_to_push.clone());
					//println!("layer===={:?}", layer);
					break;
				}
			}
		}
		else {
			let st_end:usize = self.stack.len()-1;
			for layer in self.stack.iter_mut().rev() {
				if layer.contains_key(&*current_node.operation.clone()) {
					*layer.get_mut(&*current_node.operation).unwrap() = node_to_push.clone();
					return;
				}
			}
			self.stack[st_end].insert(*current_node.operation.clone(), node_to_push.clone());
		}
	}
	
	fn parse_node_list(&mut self, mut user_return:&mut Box<StackNode>, is_scope:bool, node_list:Box<Vec<Box<StackNode>>>, is_global:bool) -> Box<Vec<Box<StackNode>>> {
		let mut ret_list:Box<Vec<Box<StackNode>>> = Box::new(vec![]);
		//boolean is passed in per layer of the stack to enable or disable execution based on
		//conditional statements
		let mut executing:Box<bool> = Box::new(true);
		
		if is_scope {
			self.stack.push(Box::new(IndexMap::new()));
			// garbage_stack.push(Box::new(vec![]));
		}
		for curr_node in node_list.iter() {
			if *user_return.ntype != NodeType::None {
				break;
			}
			let new_node = self.parse_node(&mut user_return, &mut executing, Box::new(*curr_node.clone()));
			ret_list.push(new_node);
		}
		if is_scope && !is_global {
			// for trash in garbage_stack.last().unwrap().iter() {
			// 	stack.last().unwrap().remove(&**trash);
			// }
			self.stack.pop();
			// garbage_stack.pop();
		}
		ret_list
	}
	
	fn get_variable(&mut self, key:String) -> Box<StackNode> {
		for layer in self.stack.iter().rev() {
			if layer.contains_key(&key) {
				return layer.get(&key).unwrap().clone()
			}
		}
		return Box::new(StackNode::default());
	}
	
	pub fn parse_node(&mut self, mut user_return: &mut Box<StackNode>, mut executing:&mut Box<bool>, node:Box<StackNode>) -> Box<StackNode>{
		//println!("\n{:?}", node);
		if !vec![NodeType::Condition, NodeType::Def].contains(&node.ntype) || (*node.ntype == NodeType::Condition && *node.operation == "if") {
			**executing = true;
		}
		let mut args_list:Box<Vec<Box<StackNode>>> = Box::new(vec![]);
		let mut ret_node:Box<StackNode> = Box::new(StackNode::default());
		*ret_node.ntype = NodeType::None;
		if **executing {
			if !vec![NodeType::Def, NodeType::Loop].contains(&*node.ntype.clone()) {
				args_list = self.parse_node_list(&mut user_return, false, node.args.clone(), false);
			}
			match *node.ntype {
				NodeType::Call => {
					match node.operation.as_str() {
						"" => {
							self.parse_node_list(&mut user_return, true, node.scope.clone(), true);
							//global scope
						}
						"import" => {
							match &*args_list[0].ntype {
								NodeType::Str(val) => {
									self.import_module(*val.clone());
								},
								_ => {},
							}
						}
						"print" => {
							match &*args_list[0].ntype {
								NodeType::Str(val) => print!("{val}"),
								NodeType::Int(val) => print!("{val}"),
								NodeType::Float(val) => print!("{val}"),
								NodeType::Bool(val) => print!("{val}"),
								NodeType::Vector => {
									print!("{}", self.vec_to_str(args_list[0].clone()));
								}
								_ => {},
							}
						}
						"push" => {
							self.push_to_array(node, args_list);
						}
						"pop" => {
							self.pop_from_array(node);
						}
						"len" => {
							ret_node.ntype = self.get_len(args_list);
						}
						"range" => {
							ret_node = self.get_range(args_list);
						}
						"CLI" => {
							match &*args_list[0].ntype {
								NodeType::Int(val) => {
									ret_node.ntype = Box::new(NodeType::Str(Box::new(self.cli_args[**val as usize + 1].clone())))
								},
								_ => {},
							}
						}
						"FFI" => {
							ret_node.ntype = self.foreign_function_interface(args_list);
						}
						"split" => {
							ret_node = self.split(args_list);
						}
						"rev" => {
							ret_node = self.reverse_list(args_list);
						}
						"trim" => {
							ret_node.ntype = self.remove_ws(args_list);
						}
						"replace" => {
							ret_node.ntype = self.replace(args_list);
						}
						"cat" => {
							ret_node.ntype = self.cat(args_list);
						}
						//put the math and complex functions into std/standard.rs as functions
						"add" => {
							ret_node.ntype = self.add(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"sub" => {
							ret_node.ntype = self.sub(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"mul" => {
							ret_node.ntype = self.mul(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"div" => {
							ret_node.ntype = self.div(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"and" => {
							ret_node.ntype = self.and(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"or" => {
							ret_node.ntype = self.or(args_list[0].ntype.clone(), args_list[1].ntype.clone());
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
							ret_node.ntype = self.equal(args_list[0].clone(), args_list[1].clone());
						}
						"NE" => {
							//NOT EQUAL
							ret_node.ntype = self.notequal(args_list[0].clone(), args_list[1].clone());
						}
						"G" => {
							//GREATER
							ret_node.ntype = self.greater(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"L" => {
							//LESS
							ret_node.ntype = self.less(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"GE" => {
							//GREATER OR EQUAL
							ret_node.ntype = self.greaterequal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"LE" => {
							//LESS OR EQUAL
							ret_node.ntype = self.lessequal(args_list[0].ntype.clone(), args_list[1].ntype.clone());
						}
						"read" => {
							//read from files and return contents
							//eg: read("folder/filepath.txt");
							ret_node.ntype = self.read(args_list[0].ntype.clone());
						}
						"write" => {
							//write to files
							//arg1 is the file name, arg3 is writemode (append or truncate or nothing for default(the default is truncate))
							//arg2 is the content to write to the file.
							//eg: write("folder/filepath.txt", "FulcrumRS is a cool language.", "a"|"t"|None);
							ret_node.ntype = self.filewrite(args_list[0].ntype.clone(), args_list[1].ntype.clone(), args_list[2].ntype.clone());
						}
						"input" => {
							let mut ret_val = String::new();
							io::Write:: flush(&mut io::stdout()).expect("flush failed!");
							stdin().read_line(&mut ret_val).unwrap();
							fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
								if s.ends_with(p) {
									&s[..s.len() - p.len()]
								} else {
									s
								}
							}
							*ret_node.ntype = NodeType::Str(Box::new(String::from(String::from(remove_suffix( &remove_suffix(&ret_val, "\n"), "\r")))))
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
						"STR" => {
							match &*args_list[0].ntype.clone() {
								NodeType::Str(val) => *ret_node.ntype = NodeType::Str(Box::new(String::from(format!("{val}")))),
								NodeType::Int(val) => *ret_node.ntype = NodeType::Str(Box::new(String::from(format!("{val}")))),
								NodeType::Float(val) => *ret_node.ntype = NodeType::Str(Box::new(String::from(format!("{val}")))),
								NodeType::Bool(val) => *ret_node.ntype = NodeType::Str(Box::new(String::from(format!("{val}")))),
								NodeType::Vector => {
									*ret_node.ntype = NodeType::Str(Box::new(self.vec_to_str(args_list[0].clone())));
								}
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
							let mut user_func = self.get_variable(*node.operation.clone());
							self.stack.push(Box::new(IndexMap::new()));
							for (an, arg) in args_list.iter().enumerate() {
								//println!("{}", node.operation.clone());
								let name = user_func.args[an].operation.clone();
								user_func.args[an] = arg.clone();
								user_func.args[an].operation = name;
								let st_end = self.stack.len() -1;
								self.stack[st_end].insert(*user_func.args[an].operation.clone(), arg.clone());
								//push_to_stack(user_func.args[an].clone(), arg.clone(), &mut stack, &mut garbage_stack);
							}
							let mut passable_return = Box::new(StackNode::default());
							passable_return.ntype = Box::new(NodeType::None);
							self.parse_node_list(&mut passable_return, true, user_func.scope, false);
							// for trash in garbage_stack.last().unwrap().iter() {
							// 	stack.remove(&**trash);
							// }
							self.stack.pop();
							return passable_return;
						} 
					}
				},
				NodeType::Def => {
					self.push_to_stack(node.clone(), node.clone());
					*ret_node = StackNode { operation: node.operation, ntype: node.ntype, args: node.args, scope: node.scope };
				},
				NodeType::Assign => {
					//println!("LHS +++ {:?}", node.args[0]);
					self.push_to_stack(node.args[0].clone(), args_list[1].clone());
				},
				NodeType::Variable => {
					ret_node = self.get_variable(*node.operation).clone();
				},
				NodeType::Return => {
					//*user_return.operation = String::from("return");
					if args_list.len() > 0 {
						*user_return = args_list[0].clone();
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
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Int(val) => {
									if **val > 0 {
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Float(val) => {
									if **val > 0.0 {
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Bool(val) => {
									if **val {
										self.parse_node_list(&mut user_return, true, node.scope, false);
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
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Int(val) => {
									if **val > 0 {
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Float(val) => {
									if **val > 0.0 {
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								NodeType::Bool(val) => {
									if **val {
										self.parse_node_list(&mut user_return, true, node.scope, false);
										**executing = false;
										*ret_node.ntype = NodeType::Bool(Box::new(true));
									}
								},
								_ => {},
							}
							*ret_node.ntype = NodeType::Bool(Box::new(false))
						}
						"else" => {
							self.parse_node_list(&mut user_return, true, node.scope, false);
							*ret_node.ntype = NodeType::Bool(Box::new(false))
						}
						_ => {
							*ret_node.ntype = NodeType::None
						}
					}
				},
				NodeType::Vector => {
					*ret_node.ntype = NodeType::Vector;
					ret_node.args = args_list;
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
									return self.parse_node(&mut user_return, &mut executing, args_list[1].args[val.parse::<usize>().expect("The provided string caused an invalid cast to integer.")].clone())
								},
								NodeType::Int(val) => {
									return self.parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone())
								},
								NodeType::Float(val) => {
									return self.parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone())
								},
								NodeType::Bool(val) => {
									return self.parse_node(&mut user_return, &mut executing, args_list[1].args[*val as usize].clone())
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
							self.stack.push(Box::new(IndexMap::new()));
							let alist = self.parse_node_list(&mut user_return, false, node.args.clone(), false);
							match *alist[1].ntype.clone() {
								NodeType::Str(val) => {
									for value in val.chars() {
										let mut val_for_stack = Box::new(StackNode::default());
										*val_for_stack.ntype = NodeType::Str(Box::new(String::from(value)));
										self.push_to_stack(node.args[0].clone(), val_for_stack.clone());
										self.parse_node_list(&mut user_return, true, node.scope.clone(), false);
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
										self.push_to_stack(node.args[0].clone(), val_for_stack.clone());
										self.parse_node_list(&mut user_return, true, node.scope.clone(), false);
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
										self.push_to_stack(node.args[0].clone(), value.clone());
										self.parse_node_list(&mut user_return, true, node.scope.clone(), false);
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
									println!("ERROR<loop::for>(make an issue on the git repo with this message and your source code): {:?}", self.stack);
								},
							}
							self.stack.pop();
							//**executing = false;
						}
						"while" => {
							loop {
								let alist = self.parse_node_list(&mut user_return, false, node.args.clone(), false);
								match *alist[0].ntype.clone() {
									NodeType::Bool(val) => {
										if *val {
											self.parse_node_list(&mut user_return, true, node.scope.clone(), false);
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
								self.parse_node_list(&mut user_return, true, node.scope.clone(), false);
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
							ret_node.ntype = self.contains_operator(&mut args_list[0].clone(), args_list[1].clone());
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
}