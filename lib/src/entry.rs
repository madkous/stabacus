use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

use operator::*;

#[derive(Clone,Debug)]
pub enum Entry {
	Op(Operator),
	Int(i64),
	Panic(String),
	Die,
	Pop,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Entry::Op(o)    => write!(f, "{}",  o),
			Entry::Int(n)   => write!(f, "{}",  n),
			Entry::Panic(s) => write!(f, "!{}", s),
			Entry::Die      => write!(f, "Die"),
			Entry::Pop      => write!(f, "Pop"),
		}
    }
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

