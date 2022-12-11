use crate::parts::datastructures::{StackNode, NodeType, Token};

pub fn func_call(name:String, mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(name),
		ntype: Box::new(NodeType::Call),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn func_def(name:String, mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(name),
		ntype: Box::new(NodeType::Def),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn condition_if(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::from("if")),
		ntype: Box::new(NodeType::Condition),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn condition_elif(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::from("elif")),
		ntype: Box::new(NodeType::Condition),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn condition_else(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::from("else")),
		ntype: Box::new(NodeType::Condition),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

fn ends(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	let last_sb = stack_buffer.len() - 1;
	let mut last_node = *stack_buffer[last_sb].0.clone();
	stack_buffer.pop();
	let last_sb = stack_buffer.len() - 1;
	// let last_inner = stack_buffer[last_sb].0.args.len();
	// if if last_inner >= 1 {*stack_buffer[last_sb].0.scope[last_inner as usize-1].ntype.clone()} else {NodeType::None} == NodeType::Assign {
		
	// 	stack_buffer[last_sb].0.args[last_inner].args.push(Box::new(last_node));
	// }
	if *stack_buffer[last_sb].1 {
		stack_buffer[last_sb].0.args.push(Box::new(last_node));
	}
	else {
		stack_buffer[last_sb].0.scope.push(Box::new(last_node));
	}
}

pub fn line_end(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	if vec![NodeType::Assign, NodeType::Return].contains(&stack_buffer.last().unwrap().0.ntype) {
		ends(&mut stack_buffer);
	}
}

pub fn statement_end(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	if *stack_buffer.last().unwrap().0.ntype != NodeType::Def {
		ends(&mut stack_buffer);
	}
}

pub fn scope_end(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	ends(&mut stack_buffer);
}
pub fn vec_end(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	ends(&mut stack_buffer);
}

pub fn scope_start(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	let last_sb = stack_buffer.len() - 1;
	stack_buffer[last_sb].1 = Box::new(false);
}

pub fn literal(token:Token, mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	let last_sb = stack_buffer.len() - 1;
	// let inner_len = stack_buffer[last_sb].0.scope.len() as i128;
	// if if inner_len >= 1 {*stack_buffer[last_sb].0.scope[inner_len as usize-1].ntype.clone()} else {NodeType::None} == NodeType::Assign {
	// 	stack_buffer[last_sb].0.scope.last_mut().unwrap().args.push(Box::new(StackNode {
	// 		operation: Box::new(String::new()),
	// 		ntype: match token {
	// 			Token::StringLit(val) => Box::new(NodeType::Str(val)),
	// 			Token::IntLit(val) => Box::new(NodeType::Int(val)),
	// 			Token::FloatLit(val) => Box::new(NodeType::Float(val)),
	// 			Token::BooleanLit(val) => Box::new(NodeType::Bool(val)),
	// 			_ => Box::new(NodeType::Int(Box::new(0))),
	// 		},
	// 		args: Box::new(vec![]),
	// 		scope: Box::new(vec![])
	// 	}));
	// }
	if *stack_buffer[last_sb].1 {
		match token {
			Token::StringLit(val) => stack_buffer[last_sb].0.args.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Str(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::IntLit(val) => stack_buffer[last_sb].0.args.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Int(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::FloatLit(val) => stack_buffer[last_sb].0.args.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Float(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::BooleanLit(val) => stack_buffer[last_sb].0.args.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Bool(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			_ => {},
		}
	}
	else {
		match token {
			Token::StringLit(val) => stack_buffer[last_sb].0.scope.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Str(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::IntLit(val) => stack_buffer[last_sb].0.scope.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Int(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::FloatLit(val) => stack_buffer[last_sb].0.scope.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Float(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			Token::BooleanLit(val) => stack_buffer[last_sb].0.scope.push(Box::new(StackNode {
				operation: Box::new(String::new()),
				ntype: Box::new(NodeType::Bool(val)),
				args: Box::new(vec![]),
				scope: Box::new(vec![]),
			})),
			_ => {},
		}
	}
}

pub fn variable(name:String, mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	let last_sb = stack_buffer.len() - 1;
	if *stack_buffer[last_sb].1 {
		stack_buffer[last_sb].0.args.push(Box::new(StackNode {
			operation: Box::new(name),
			ntype: Box::new(NodeType::Variable),
			args: Box::new(vec![]),
			scope: Box::new(vec![]),
		}));
	}
	else {
		stack_buffer[last_sb].0.scope.push(Box::new(StackNode {
			operation: Box::new(name),
			ntype: Box::new(NodeType::Variable),
			args: Box::new(vec![]),
			scope: Box::new(vec![]),
		}));
	}
}

pub fn assign(mut stack_buffer:&mut Vec<(Box<StackNode>, Box<bool>)>) {
	
	let last_sb = stack_buffer.len() - 1;
	let last_in_last_children = stack_buffer[last_sb].0.scope.last().unwrap().clone();
	stack_buffer[last_sb].0.scope.pop();
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::new()),
		ntype: Box::new(NodeType::Assign),
		args: Box::new(vec![last_in_last_children]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
	// let last_sb = stack_buffer.len() - 1;
	// let last_node = stack_buffer[last_sb].0.clone();
	// stack_buffer.pop();
	// stack_buffer.push((Box::new(StackNode {
	// 	operation: Box::new(String::new()),
	// 	ntype: Box::new(NodeType::Assign),
	// 	args: Box::new(vec![last_node]),
	// 	scope: Box::new(vec![]),
	// }), Box::new(true)));
	// println!("{:?}", stack_buffer);
}

pub fn return_val(mut stack_buffer:&mut Box<Vec<(Box<StackNode>, Box<bool>)>>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::new()),
		ntype: Box::new(NodeType::Return),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn vector(mut stack_buffer:&mut Box<Vec<(Box<StackNode>, Box<bool>)>>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::new()),
		ntype: Box::new(NodeType::Vector),//(Box::new(vec![])),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}
pub fn index(mut stack_buffer:&mut Box<Vec<(Box<StackNode>, Box<bool>)>>) {
	stack_buffer.push((Box::new(StackNode {
		operation: Box::new(String::new()),
		ntype: Box::new(NodeType::Index),
		args: Box::new(vec![]),
		scope: Box::new(vec![]),
	}), Box::new(true)));
}

pub fn end_index(mut stack_buffer:&mut Box<Vec<(Box<StackNode>, Box<bool>)>>) {
	let last_sb = stack_buffer.len() - 1;
	let mut last_node = *stack_buffer[last_sb].0.clone();
	stack_buffer.pop();
	let last_sb = stack_buffer.len() - 1;
	if *stack_buffer[last_sb].1 {
		let last_in_last_children = stack_buffer[last_sb].0.args.last().unwrap().clone();
		last_node.args.push(last_in_last_children);
		stack_buffer[last_sb].0.args.pop();
		stack_buffer[last_sb].0.args.push(Box::new(last_node));
	}
	else {
		let last_in_last_children = stack_buffer[last_sb].0.scope.last().unwrap().clone();
		last_node.args.push(last_in_last_children);
		stack_buffer[last_sb].0.scope.pop();
		stack_buffer[last_sb].0.scope.push(Box::new(last_node));
	}
}