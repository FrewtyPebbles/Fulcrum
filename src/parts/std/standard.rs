use std::{fs::{self, OpenOptions, File}, path::PathBuf};

use indexmap::IndexMap;

use crate::parts::datastructures::{NodeType, StackNode};

pub fn add(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
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

pub fn and(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
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

pub fn or(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
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

pub fn sub(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
			NodeType::Str(val2) => Box::new(NodeType::Str(Box::new(val.replace(&*val2, "")))),
			NodeType::Int(val2) => {
				let mut ret = val.clone();
				for _ in 0..*val2 {
					val.pop();
				}
				return Box::new(NodeType::Str(Box::new(*val)));
			},
			NodeType::Float(val2) => {
				let mut ret = val.clone();
				for _ in 0..*val2 as i128 {
					val.pop();
				}
				return Box::new(NodeType::Str(Box::new(*val)));
			},
			NodeType::Bool(val2) => {
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

pub fn mul(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
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
			NodeType::Bool(val2) => {
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

pub fn div(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
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
			NodeType::Bool(val2) => {
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

pub fn read(mut filepath:Box<NodeType>, origin:String) -> Box<NodeType> {
	let path = PathBuf::from(origin);
	let dir = path.parent().unwrap();
	match *filepath {
		NodeType::Str(val) => {
			Box::new(NodeType::Str(Box::new(fs::read_to_string(fs::canonicalize(dir).unwrap().join(*val)).unwrap())))
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}
pub fn filewrite(mut filepath:Box<NodeType>, mut content:Box<NodeType>, mut writemode:Box<NodeType>, origin:String) -> Box<NodeType> {
	let path = PathBuf::from(origin);
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
							match openopt.open(fs::canonicalize(dir).unwrap().join(*filep)) {
								Ok(val) => {
									file = val;
									write!(file, "{}", cont);
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

pub fn equal(mut lhs:Box<StackNode>, mut rhs:Box<StackNode>) -> Box<NodeType> {
	match *lhs.ntype {
		NodeType::Str(mut val) => match *rhs.ntype {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val == if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val == if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val == *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
		NodeType::Vector => match *rhs.ntype {
			NodeType::Vector => Box::new(NodeType::Bool(Box::new(lhs.args.iter().zip(&*rhs.args).all(|(a, b)| **a == **b)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn notequal(mut lhs:Box<StackNode>, mut rhs:Box<StackNode>) -> Box<NodeType> {
	match *lhs.ntype {
		NodeType::Str(mut val) => match *rhs.ntype {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val != if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val != if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val != *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		NodeType::Vector => match *rhs.ntype {
			NodeType::Vector => Box::new(NodeType::Bool(Box::new(!lhs.args.iter().zip(&*rhs.args).all(|(a, b)| **a == **b)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		}
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn greater(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val > if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val > if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val > *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn less(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val < if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val < if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val < *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn lessequal(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val <= if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val <= if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val <= *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn greaterequal(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
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
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new((*val >= if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new((*val >= if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new((*val >= *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}

pub fn contains_operator(mut lhs:&mut Box<StackNode>, mut rhs:Box<StackNode>) -> Box<NodeType> {
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

pub fn in_operator(mut lhs:&mut Box<StackNode>, mut rhs:Box<StackNode>) -> Box<NodeType> {
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

pub fn cat(mut args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
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

pub fn foreign_function_interface(mut args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
	return Box::new(NodeType::Str(Box::new(String::new())));
}

pub fn split(mut args_list:Box<Vec<Box<StackNode>>>) -> Box<StackNode> {
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

pub fn remove_ws(mut args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
	return Box::new(NodeType::Str(Box::new(match *args_list[0].ntype.clone() {
		NodeType::Str(val) => String::from(val.trim()),
		_ => {String::new()},
	})));
}

pub fn replace(mut args_list:Box<Vec<Box<StackNode>>>) -> Box<NodeType> {
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

pub fn push_to_array(mut args_list:Box<Vec<Box<StackNode>>>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>) {
	for layer in stack.iter_mut().rev() {
		if layer.contains_key(args_list[0].args[0].operation.clone().as_str()) {
			match *layer.get(&*args_list[0].args[0].operation).unwrap().ntype.clone() {
				NodeType::Str(val) => {
					match *args_list[1].ntype.clone() {
						NodeType::Str(val2) => {
							layer.get_mut(&*args_list[0].args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
						},
						NodeType::Int(val2) => {
							layer.get_mut(&*args_list[0].args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
						},
						NodeType::Float(val2) => {
							layer.get_mut(&*args_list[0].args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
						},
						NodeType::Bool(val2) => {
							layer.get_mut(&*args_list[0].args[0].operation).unwrap().ntype = Box::new(NodeType::Str(Box::new(format!("{val}{val2}"))));
						},
						_ => {

						}
					}
				}
				NodeType::Vector => {
					layer.get_mut(&*args_list[0].args[0].operation).unwrap().args.push(args_list[1].clone());
				}
				_ => {

				}
			}
		}
	}
}

pub fn pop_from_array(mut args_list:Box<Vec<Box<StackNode>>>, mut stack:&mut Box<Vec<Box<IndexMap<String, Box<StackNode>>>>>) {
	for layer in stack.iter_mut().rev() {
		if layer.contains_key(args_list[0].args[0].operation.clone().as_str()) {
			match *layer.get(&*args_list[0].args[0].operation).unwrap().ntype.clone() {
				NodeType::Str(mut val) => {
					val.pop();
					layer.get_mut(&*args_list[0].args[0].operation).unwrap().ntype = Box::new(NodeType::Str(val));
				}
				NodeType::Vector => {
					layer.get_mut(&*args_list[0].args[0].operation).unwrap().args.pop();
				}
				_ => {

				}
			}
		}
	}
}