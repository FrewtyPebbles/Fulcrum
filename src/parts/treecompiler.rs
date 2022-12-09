use crate::parts::{tcdefinitions::definitions::{return_val, scope_start}, parser::parse_tree};

use super::{datastructures::{Token, StackNode, NodeType}, tcdefinitions::definitions::{func_def, func_call, condition_if, condition_elif, condition_else, statement_end, assign, variable, literal, line_end, scope_end, vector, index, vec_end, end_index}};



pub fn compile_tree(tokenlist:Vec<Token>) {
	// tuple 0 = the node and tuple 1 = wether we are in the args or scope of that node
	let mut stack_buffer:Box<Vec<(Box<StackNode>, Box<bool>)>> = Box::new(vec![(Box::new(StackNode::default()), Box::new(false))]);
	for (tn, current_token) in tokenlist.iter().enumerate()
	{
		//println!("token number: {tn}");
		match current_token {
			Token::FuncCall(name) => {
				func_call(*name.clone(), &mut stack_buffer);
			},
			Token::FuncDeff(name) => {
				func_def(*name.clone(), &mut stack_buffer);
			},
			Token::If => {
				condition_if(&mut stack_buffer);
			},
			Token::ElIf => {
				condition_elif(&mut stack_buffer);
			},
			Token::El => {
				condition_else(&mut stack_buffer);
			},
			Token::StatementEnd => {
				statement_end(&mut stack_buffer);
			},
			Token::StringLit(_) => {
				literal(current_token.clone(), &mut stack_buffer);
			},
			Token::IntLit(_) => {
				literal(current_token.clone(), &mut stack_buffer);
			},
			Token::FloatLit(_) => {
				literal(current_token.clone(), &mut stack_buffer);
			},
			Token::BooleanLit(_) => {
				literal(current_token.clone(), &mut stack_buffer);
			},
			Token::StartScope => {
				scope_start(&mut stack_buffer);
			},
			Token::EndScope => {
				scope_end(&mut stack_buffer);
			},
			Token::Variable(name) => {
				variable(*name.clone(), &mut stack_buffer);
			},
			Token::Assign(name) => {
				assign(*name.clone(), &mut stack_buffer);
			},
			Token::Return => {
				return_val(&mut stack_buffer);
			},
			Token::EndLine => {
				line_end(&mut stack_buffer);
			},
			Token::StartVec => {
				vector(&mut stack_buffer);
			},
			Token::EndVec => {
				vec_end(&mut stack_buffer);
			},
			Token::IndexStart => {
				index(&mut stack_buffer);
			},
			Token::IndexEnd => {
				end_index(&mut stack_buffer);
			}
			Token::Delimeter => {},
			Token::Void => {
				
			},
		}
	}
	//println!("{:?}", stack_buffer);
	parse_tree(stack_buffer[0].0.clone());
}