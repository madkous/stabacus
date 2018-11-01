use std::io;
use std::fmt;
use std::process;

// use stack::*;

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
	match v {
		&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x+y),
		_ => Entry::Panic(format!{"bad args: {:?}", v}),
	}
}

pub fn bin_times(v: &[Entry]) -> Entry {
	match v {
		&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x*y),
		_ => Entry::Panic(format!{"bad args: {:?}", v}),
	}
}

pub fn bin_minus(v: &[Entry]) -> Entry {
	match v {
		&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x-y),
		_ => Entry::Panic(format!{"bad args: {:?}", v}),
	}
}

pub fn bin_divide(v: &[Entry]) -> Entry {
	match v {
		&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x/y),
		_ => Entry::Panic(format!{"bad args: {:?}", v}),
	}
}

pub fn bin_remainder(v: &[Entry]) -> Entry {
	match v {
		&[Entry::Int(x), Entry::Int(y)] => Entry::Int(x%y),
		_ => Entry::Panic(format!{"bad args: {:?}", v}),
	}
}

pub fn quit(_v: &[Entry]) -> Entry {
	process::exit(0);
}

// fn sum(n: usize) -> fn(&[i64]) -> i64 {

