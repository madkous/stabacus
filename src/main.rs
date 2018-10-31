use std::io;
use std::str::FromStr;
use std::string::ParseError;
use std::collections::HashMap;

extern crate lib;
use entry::*;
use stack::*;

#[derive(Debug)]
enum ParseType {
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

impl ParseType {
	fn get_entry(self, ops: &HashMap<String,Operator>) -> Entry {
		match self {
			ParseType::Int(z)   => Entry::Int(z),
			ParseType::Str(s)   => match ops.get(&s) {
				Some(o) => Entry::Op(o.clone()),
				None    => Entry::Panic(format!("Unknown Operator: {}", s)),
			},
		}
	}
}

fn main() {
	println!("Stack Based RPN Calculator:");

	let mut stack: Stack = Stack::new();
	let mut operators = HashMap::new();
	operators.insert("+".to_string(), Operator { name: "+".to_string(), arity: 2, body: bin_plus });

	loop {
		let mut s = String::new();
		io::stdin().read_line(&mut s)
			.expect("Failed to read line.");

		let i: Entry = match s.trim().parse::<ParseType>()
			.map(|i| ParseType::get_entry(i, &operators)) {
				Ok(val) => val,
				Err(_) => continue,
			};

		match i {
			Entry::Op(_) => stack.push(i).operate(),
			Entry::Int(_) => stack.push(i),
			Entry::Panic(s) => panic!(s),
		};

		println!("stack: {:?}", stack);
	}
}
