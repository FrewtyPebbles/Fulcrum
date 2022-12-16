#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	FuncCall(Box<String>),
	FuncDeff(Box<String>),
	If,
	ElIf,
	El,
	StatementEnd,
	StringLit(Box<String>),
	IntLit(Box<i128>),
	FloatLit(Box<f64>),
	BooleanLit(Box<bool>),
	StartScope,
	EndScope,
	StartVec,
	EndVec,
	Variable(Box<String>),
	IndexStart,
	IndexEnd,
	Assign,
	Return,
	EndLine,
	Delimeter,
	For,
	In,
	While,
	Loop,
	Break,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
	Call,
	Def,
	Assign,
	Variable,
	Return,
	Condition,
	Str(Box<String>),
	Int(Box<i128>),
	Float(Box<f64>),
	Bool(Box<bool>),
	//vector holds variables in the arguments
	Vector,//(Box<Vec<Box<NodeType>>>),
	//argument 1 is the value argument 2 is the index
	Index,
	Loop,
	Operator,
	Break,
	None
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackNode {
	pub operation: Box<String>,
	pub ntype:Box<NodeType>,
	pub args:Box<Vec<Box<StackNode>>>,
	pub scope:Box<Vec<Box<StackNode>>>
}

impl Default for StackNode {
    fn default() -> Self {
        Self { operation: Default::default(), ntype: Box::new(NodeType::Call), args: Default::default(), scope: Default::default() }
    }
}