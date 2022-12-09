use std::fs;

use crate::parts::datastructures::NodeType;

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

pub fn read(mut filepath:Box<NodeType>) -> Box<NodeType> {
	match *filepath {
		NodeType::Str(val) => {
			Box::new(NodeType::Str(Box::new(fs::read_to_string(*val).unwrap())))
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}
pub fn write(mut filepath:Box<NodeType>, mut content:Box<NodeType>, mut writemode:Box<NodeType>) -> Box<NodeType> {
	Box::new(NodeType::Bool(Box::new(false)))
}

pub fn equal(mut lhs:Box<NodeType>, mut rhs:Box<NodeType>) -> Box<NodeType> {
	match *lhs {
		NodeType::Str(mut val) => match *rhs {
			NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2))),
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == format!("{val2}")))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		NodeType::Int(val) => match *rhs {
			NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 == format!("{val}")))),
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as i128))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as i128))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		NodeType::Float(val) => match *rhs {
			NodeType::Str(val2) => Box::new(NodeType::Bool(Box::new(*val2 == format!("{val}")))),
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as f64))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(*val == *val2 as f64))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(*val == if *val2 {1.0} else {0.0}))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		NodeType::Bool(val) => match *rhs {
			NodeType::Str(val2) => Box::new(NodeType::Str(Box::new((*val2 == format!("{val}")).to_string()))),
			NodeType::Int(val2) => Box::new(NodeType::Bool(Box::new(!(*val == if *val2 >= 1 {true} else {false})))),
			NodeType::Float(val2) => Box::new(NodeType::Bool(Box::new(!(*val == if *val2 >= 1.0 {true} else {false})))),
			NodeType::Bool(val2) => Box::new(NodeType::Bool(Box::new(!(*val == *val2)))),
			_ => {Box::new(NodeType::Bool(Box::new(false)))},
		},
		_ => {Box::new(NodeType::Bool(Box::new(false)))},
	}
}