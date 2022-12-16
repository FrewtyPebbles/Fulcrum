use std::{fs::{self, OpenOptions, File}, path::PathBuf};

use crate::parts::{datastructures::{NodeType, StackNode}, parser::Parser, tokenizer::tokenize};

impl Parser {
	pub fn add(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Int(Box::new(*val + *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Int(Box::new(*val + *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Int(Box::new(*val + *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Float(Box::new(*val + *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Float(Box::new(*val + *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Float(Box::new(*val + if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val && if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val && if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val && *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn and(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 && *val2 != 0))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 && *val2 != 0.0))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 && *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0 && *val2 != 0))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0 && *val2 != 0.0))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0&& *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val && if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val && if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val && *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn or(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 || *val2 != 0))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 || *val2 != 0.0))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != 0 || *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0 || *val2 != 0))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0 || *val2 != 0.0))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != 0.0 || *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val || if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val || if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val || *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn sub(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(mut val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(val.replace(&*val2, "")))),
				NodeType::Int(val2) => {
					for _ in 0..*val2 {
						val.pop();
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Float(val2) => {
					for _ in 0..*val2 as i128 {
						val.pop();
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Bool(_) => {
					Box::new(NodeType::Bool(Box::new(false)))
				},
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Int(Box::new(*val - *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Int(Box::new(*val - *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Int(Box::new(*val - *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Float(Box::new(*val - *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Float(Box::new(*val - *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Float(Box::new(*val - if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(!(*val && if *val2 >= 1 {true} else {false})))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(!(*val && if *val2 >= 1.0 {true} else {false})))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(!(*val && *val2)))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn mul(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(mut val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(val.replace(&*val2, "")))),
				NodeType::Int(val2) => {
					let ret = val.clone();
					for _ in 0..*val2 {
						*val = format!("{ret}{ret}");
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Float(val2) => {
					let ret = val.clone();
					for _ in 0..*val2 as i128 {
						*val = format!("{ret}{ret}");
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Bool(_) => {
					Box::new(NodeType::Bool(Box::new(false)))
				},
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Int(Box::new(*val * *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Int(Box::new(*val * *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Int(Box::new(*val * *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Float(Box::new(*val * *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Float(Box::new(*val * *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Float(Box::new(*val * if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val || if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val || if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val || *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn div(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(mut val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(val.replace(&*val2, "")))),
				NodeType::Int(val2) => {
					let ret = val.clone();
					for _ in 0..*val2 {
						*val = format!("{ret}{ret}");
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Float(val2) => {
					let ret = val.clone();
					for _ in 0..*val2 as i128 {
						*val = format!("{ret}{ret}");
					}
					return Box::new(NodeType::Str(Box::new(*val)));
				},
				NodeType::Bool(_) => {
					Box::new(NodeType::Bool(Box::new(false)))
				},
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Int(Box::new(*val / *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Int(Box::new(*val / *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Int(Box::new(*val / *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Float(Box::new(*val / *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Float(Box::new(*val / *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Float(Box::new(*val / if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(format!("{val}{val2}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(!(*val || if *val2 >= 1 {true} else {false})))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(!(*val || if *val2 >= 1.0 {true} else {false})))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(!(*val || *val2)))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn read(&mut self, filepath:Box<NodeType>) -> Box<NodeType> {
		let path = PathBuf::from(self.file_path.clone());
		let dir = path.parent().unwrap();
		
		match *filepath {
			NodeType::Str(val) => {
				//println!("{}", dir.join(*val.clone()).display());
				Box::new(NodeType::Str(Box::new(fs::read_to_string(dir.join(*val)).unwrap())))
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}
	pub fn filewrite(&mut self, filepath:Box<NodeType>, content:Box<NodeType>, writemode:Box<NodeType>) -> Box<NodeType> {
		let path = PathBuf::from(self.file_path.clone());
		let dir = path.parent().unwrap();
		match *filepath.clone() {
			NodeType::Str(filep) => {
				match *writemode.clone() {
					NodeType::Str(wm) => {
						match *content.clone() {
							NodeType::Str(cont) => {
								use std::io::Write;
								let mut file:File;
								let mut openopt:&mut OpenOptions = &mut OpenOptions::new();
								match &*wm.to_lowercase().as_str() {
									"a" => {
										openopt = openopt.write(true).create(true).append(true);
									}
									"t" => {
										openopt = openopt.write(true).create(true).truncate(true);
									}
									_ => {}
								}
								match openopt.open(dir.join(*filep)) {
									Ok(val) => {
										file = val;
										write!(file, "{}", cont).ok();
									},
									Err(val) => {
										eprintln!("{val}");
									},
								}
							},
							_ => {},
						}
					},
					_ => {},
				}
			},
			_ => {},
		}
		
		Box::new(NodeType::Bool(Box::new(false)))
	}

	pub fn equal(&mut self, lhs:Box<StackNode>, rhs:Box<StackNode>) -> Box<NodeType> {
		match *lhs.ntype {
			NodeType::Str(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 == format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 == format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 == format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			}
			NodeType::Vector => match *rhs.ntype {
				NodeType::Vector => Box::new(NodeType::Bool(Box::new(lhs.args.iter().zip(&*rhs.args).all(|(a, b)| **a == **b)))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			}
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn notequal(&mut self, lhs:Box<StackNode>, rhs:Box<StackNode>) -> Box<NodeType> {
		match *lhs.ntype {
			NodeType::Str(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 != format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 != format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs.ntype {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 != format!("{val}")).to_string()))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val != if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val != if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val != *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Vector => match *rhs.ntype {
				NodeType::Vector => Box::new(NodeType::Bool(Box::new(!lhs.args.iter().zip(&*rhs.args).all(|(a, b)| **a == **b)))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			}
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn greater(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val > format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val > format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val > format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 > format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 > format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val > if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 > format!("{val}")).to_string()))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val > if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val > if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val > *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn less(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val < format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val < format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val < format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 < format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 < format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val < if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 < format!("{val}")).to_string()))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val < if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val < if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val < *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn lessequal(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val <= format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val <= format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val <= format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 <= format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 <= format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val <= if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 <= format!("{val}")).to_string()))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val <= if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val <= if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val <= *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn greaterequal(&mut self, lhs:Box<NodeType>, rhs:Box<NodeType>) -> Box<NodeType> {
		match *lhs {
			NodeType::Str(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val >= format!("{val2}")))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val >= format!("{val2}")))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val >= format!("{val2}")))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Int(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 >= format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2 as i128))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2 as i128))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Float(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 >= format!("{val}")))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2 as f64))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2 as f64))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val >= if *val2 {1.0} else {0.0}))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			NodeType::Bool(val) => match *rhs {
				NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 >= format!("{val}")).to_string()))),
				NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val >= if *val2 >= 1 {true} else {false}))),
				NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val >= if *val2 >= 1.0 {true} else {false}))),
				NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val >= *val2))),
				_ => {Box::new(NodeType::Bool(Box::new(false)))},
			},
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
	}

	pub fn contains_operator(&mut self, lhs:&mut Box<StackNode>, rhs:Box<StackNode>) -> Box<NodeType> {
		match *rhs.ntype.clone() {
			NodeType::Str(val) => {
				match *lhs.ntype.clone() {
					NodeType::Str(val2) => {
						return Box::new(NodeType::Bool(Box::new(val.contains(&*val2))));
					}
					NodeType::Int(val2) => {
						return Box::new(NodeType::Bool(Box::new(val.contains(&format!("{val2}")))));
					}
					NodeType::Float(val2) => {
						return Box::new(NodeType::Bool(Box::new(val.contains(&format!("{val2}")))));
					}
					_ => {

					}
				}
			},
			NodeType::Vector => {
				for item in rhs.args.iter() {
					if item == lhs {
						return Box::new(NodeType::Bool(Box::new(false)));
					}
				}
			},
			_ => todo!(),
		}
		Box::new(NodeType::Bool(Box::new(false)))
	}

	pub fn cat(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
		if args_list.len() == 0 {
			println!("Meow!");
		}
		let mut ret = String::new();
		for arg in args_list.iter() {
			match *arg.ntype.clone() {
				NodeType::Str(val) => ret = format!("{ret}{val}"),
				NodeType::Int(val) => ret = format!("{ret}{val}"),
				NodeType::Float(val) => ret = format!("{ret}{val}"),
				NodeType::Bool(val) => ret = format!("{ret}{val}"),
				_ => {},
			}
		}
		return Box::new(NodeType::Str(Box::new(ret)));
	}

	pub fn foreign_function_interface(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
		return Box::new(NodeType::Str(Box::new(String::new())));
	}

	pub fn split(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<StackNode> {
		let mut vector = StackNode {
			operation: Box::new(String::new()),
			ntype: Box::new(NodeType::Vector),
			args: Box::new(vec![]),
			scope: Box::new(vec![]),
		};
		match *args_list[0].ntype.clone() {
			NodeType::Str(val) => match *args_list[1].ntype.clone() {
				NodeType::Str(val2) => {
					for substr in val.split(val2.as_str()) {
						vector.args.push(Box::new(StackNode {
							operation: Box::new(String::new()),
							ntype: Box::new(NodeType::Str(Box::new(String::from(substr)))),
							args: Box::new(vec![]),
							scope: Box::new(vec![]),
						}));
					}
				},
				_ => {},
			},
			_ => {},
		}
		return Box::new(vector);
	}

	pub fn remove_ws(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
		return Box::new(NodeType::Str(Box::new(match *args_list[0].ntype.clone() {
			NodeType::Str(val) => String::from(val.trim()),
			_ => {String::new()},
		})));
	}

	pub fn replace(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
		return Box::new(NodeType::Str(Box::new(match *args_list[0].ntype.clone() {
			NodeType::Str(val) => {
				match *args_list[1].ntype.clone() {
					NodeType::Str(val1) => {
						match *args_list[2].ntype.clone() {
							NodeType::Str(val2) => {
								String::from(val.replace(val1.as_str(), val2.as_str()))
							},
							_ => {String::new()},
						}
					},
					_ => {String::new()},
				}
			},
			_ => {String::new()},
		})));
	}

	pub fn push_to_array(&mut self, node:Box<StackNode>, args_list:Box<Vec<Box<StackNode>>>) {
		for layer in self.stack.iter_mut().rev() {
			if layer.contains_key(node.args[0].operation.clone().as_str()) {
				match *layer.get(&*node.args[0].operation).unwrap().ntype.clone() {
					NodeType::Str(val) => {
						match *args_list[1].ntype.clone() {
							NodeType::Str(val2) => {
								layer.get_mut(&*node.args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
							},
							NodeType::Int(val2) => {
								layer.get_mut(&*node.args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
							},
							NodeType::Float(val2) => {
								layer.get_mut(&*node.args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
							},
							NodeType::Bool(val2) => {
								layer.get_mut(&*node.args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
							},
							_ => {

							}
						}
					}
					NodeType::Vector => {
						layer.get_mut(&*node.args[0].operation).unwrap().args.push(args_list[1].clone());
					}
					_ => {

					}
				}
			}
		}
	}

	pub fn pop_from_array(&mut self, node:Box<StackNode>) {
		for layer in self.stack.iter_mut().rev() {
			if layer.contains_key(node.args[0].operation.clone().as_str()) {
				match *layer.get(&*node.args[0].operation).unwrap().ntype.clone() {
					NodeType::Str(mut val) => {
						val.pop();
						layer.get_mut(&*node.args[0].operation).unwrap().ntype = Box::new(NodeType::Str(val));
					}
					NodeType::Vector => {
						layer.get_mut(&*node.args[0].operation).unwrap().args.pop();
					}
					_ => {

					}
				}
			}
		}
	}

	pub fn get_len(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
		match *args_list[0].ntype.clone() {
			NodeType::Str(val) => {
				return Box::new(NodeType::Int(Box::new(val.len() as i128)));
			},
			NodeType::Vector => {
				return Box::new(NodeType::Int(Box::new(args_list[0].args.len() as i128)));
			},
			_ => {
				eprintln!("Invalid type");
			}
		}
		Box::new(NodeType::None)
	}

	pub fn get_range(&mut self, args_list:Box<Vec<Box<StackNode>>>) -> Box<StackNode> {
		let mut vector = StackNode {
			operation: Box::new(String::new()),
			ntype: Box::new(NodeType::Vector),
			args: Box::new(vec![]),
			scope: Box::new(vec![]),
		};
		match *args_list[0].ntype.clone() {
			NodeType::Int(val1) => {
				match *args_list[1].ntype.clone() {
					NodeType::Int(val2) => {
						for i in *val1..*val2 {
							vector.args.push(Box::new(StackNode {
								operation: Box::new(String::new()),
								ntype: Box::new(NodeType::Int(Box::new(i))),
								args: Box::new(vec![]),
								scope: Box::new(vec![]),
							}));
						}
					},
					_ => {
						eprintln!("Invalid type");
					}
				}
			},
			_ => {
				eprintln!("Invalid type");
			}
		}
		Box::new(vector)
	}

	pub fn vec_to_str(&mut self, vect:Box<StackNode>) -> String {
		let mut return_string = String::new();
		return_string += "[";
		for node in vect.args.iter() {
			match *node.ntype.clone() {
				NodeType::Str(val) => {
					return_string += &String::from(format!("{:?}", val));
				},
				NodeType::Int(val) => {
					return_string += &String::from(format!("{val}"));
				},
				NodeType::Float(val) => {
					return_string += &String::from(format!("{val}"));
				},
				NodeType::Bool(val) => {
					return_string += &String::from(format!("{val}"));
				},
				NodeType::Vector => {
					return_string += &self.vec_to_str(node.clone());
				},
				_ => todo!(),
			}
			return_string += ", ";
		}
		if return_string.len() > 2 {
			return_string.pop();
			return_string.pop();
		}
		return_string += "]";
		return_string
	}

	pub fn import_module(&mut self, file:String) {
		let path = PathBuf::from(self.file_path.clone());
		let dir = path.parent().unwrap();
		let contents = fs::read_to_string(dir.join(file.clone()))
			.expect("Invalid filepath");
		self.stack.last_mut().unwrap().extend(tokenize(contents, file, self.cli_args.clone()).into_iter());
		//println!("{:?}",self.stack)
	}
	pub fn reverse_list(&mut self, mut args_list:Box<Vec<Box<StackNode>>>) -> Box<StackNode> {
		match *args_list[0].ntype.clone() {
			NodeType::Str(val) =>{
				*args_list[0].ntype = NodeType::Str(Box::new(val.chars().rev().collect::<String>()));
			},
			NodeType::Vector => {
				args_list[0].args.reverse();
			}
			_ => {},
		}
		args_list[0].clone()
	}
}
