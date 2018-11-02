// use std::io;
use std::fmt;
use std::process;
use std::collections::HashMap;

extern crate proc_macro;
// use syn::Ident;

use entry::*;

#[derive(Debug)]
pub struct OpMap(HashMap<String,Operator>);

type Sfunc = (fn(&[Entry]) -> Option<Entry>);

#[derive(Clone)]
pub struct Operator {
	pub name: String,
	pub arity: usize,
	pub body: Sfunc,
}

macro_rules! opmap {
	( $(($n:expr , $a:expr, $b:ident)),* ) => {
		{
			let mut temp_map = OpMap(HashMap::new());
			$(
				temp_map.add_op($n, $a, $b);
			)*
			temp_map
		}
	};
}

impl fmt::Debug for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Op {}:{}", self.name, self.arity)
	}
}

impl OpMap {
	pub fn default() -> OpMap {
		opmap![("+",    2, bin_plus),
		       ("+",    2, bin_plus),
		       ("*",    2, bin_times),
		       ("-",    2, bin_minus),
		       ("/",    2, bin_divide),
		       ("%",    2, bin_remainder),
		       ("sum",  1, sum),
		       ("prod", 1, prod),
		       ("q",    0, quit),
		       ("pop",  1, pop)]
	}

	pub fn add_op(&mut self, s: &str, a: usize, b: Sfunc) {
		self.insert(s.to_string(),
		            Operator { name: s.to_string(), arity: a, body: b });
	}

	pub fn get(&self, s: &String) -> Option<&Operator> {
		self.0.get(s)
	}

	pub fn insert(&mut self, s: String, op: Operator) -> Option<Operator> {
		self.0.insert(s, op)
	}

	pub fn get_entry(&self, p: ParseType) -> Entry {
		match p {
			ParseType::Int(z)   => Entry::Int(z),
			ParseType::Str(s)   => match self.get(&s) {
				Some(o) => Entry::Op(o.clone()),
				None    => Entry::Panic(format!("Unknown Operator: {}", s)),
			},
		}
	}
}

macro_rules! bin_func {
	( $op:tt, $n:ident ) => {
		fn $n(v: &[Entry]) -> Option<Entry> {
			Some(match v {
				&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x $op y),
				_ => Entry::Panic(format!("bad args: {:?}", v)),
			})
		}
	};
}

bin_func!(+, bin_plus);
bin_func!(*, bin_times);
bin_func!(-, bin_minus);
bin_func!(/, bin_divide);
bin_func!(%, bin_remainder);

macro_rules! nary_func { // TODO: get rid of name
	( $op:tt, $id:expr, $i:ident, $n:expr) => {
		fn $i(v: &[Entry]) -> Option<Entry> {
			Some(match v {
				&[Entry::Int(n)] =>
					Entry::Op(Operator { name: format!("{}{}", $n, n), arity: n as usize,
					body: |u: &[Entry]| {
						let mut i = u.iter();
						let mut c: i64 = match i.next() {
							Some(Entry::Int(m)) => *m,
							x => return Some(Entry::Panic(format!("bad arg: {:?} in {:?}", x, u)))
						};
						for x in i {
							if let Entry::Int(m) = x {
								c = c $op m;
							} else {
								return Some(Entry::Panic(format!("bad arg: {:?} in {:?}", x, u)));
							}
						}
						Some(Entry::Int(c))
					}}),
				_ => Entry::Panic(format!("bad args: {:?}", v)),
			})
		}
	};
}

nary_func!(+, 1, sum, "sum");
nary_func!(*, 0, prod, "prod");

fn quit(_v: &[Entry]) -> Option<Entry> {
	process::exit(0);
}

fn pop(_v: &[Entry]) -> Option<Entry> {
	None
}

