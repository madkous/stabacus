use std::str::FromStr;
use std::string::ParseError;

use operator::*;

#[derive(Clone,Debug)]
pub enum Entry {
	Op(Operator),
	Int(i64),
	Panic(String),
}

#[derive(Debug)]
pub enum ParseType {
	Str(String),
	Int(i64),
}

impl FromStr for ParseType {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.parse::<i64>() {
			Ok(n)  => Ok(ParseType::Int(n)),
			Err(_) => Ok(ParseType::Str(s.to_string())),
		}
	}
}

// impl ParseType {
// 	fn get_entry(self, ops: &HashMap<String,Operator>) -> Entry {
// 		match self {
// 			ParseType::Int(z)   => Entry::Int(z),
// 			ParseType::Str(s)   => match ops.get(&s) {
// 				Some(o) => Entry::Op(o.clone()),
// 				None    => Entry::Panic(format!("Unknown Operator: {}", s)),
// 			},
// 		}
// 	}
// }


// fn sum(n: usize) -> fn(&[i64]) -> i64 {

