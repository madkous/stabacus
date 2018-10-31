use std::io;
use std::fmt;

use stack::*;

type Sfunc = fn(&[Entry]) -> Entry;

#[derive(Clone,Debug)]
pub enum Entry {
	Op(Operator),
	Int(i64),
	Panic(String),
}

#[derive(Clone)]
pub struct Operator {
	pub name: String,
	pub arity: usize,
	pub body: Sfunc,
}

impl fmt::Debug for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Operator {}:{}", self.name, self.arity)
	}
}

pub fn bin_plus(v: &[Entry]) -> Entry {
	// v[0] + v[1]
	Entry::Int(5)
}

// fn sum(n: usize) -> fn(&[i64]) -> i64 {

