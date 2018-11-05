// Copyright 2018 Matthew Kousoulas
// This file is part of Stabacus.
//
// Stabacus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Stabacus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stabacus.  If not, see <https://www.gnu.org/licenses/>.
//
// @license GPL-3.0-or-later <http://spdx.org/licenses/GPL-3.0-or-later>
use std::fmt;
use std::collections::{BTreeMap, btree_map::Values};

use entry::*;

#[derive(Debug)]
pub struct OpMap(BTreeMap<String,Operator>);

type Sfunc = (fn(&[Entry]) -> Vec<Entry>);

#[derive(Clone)]
pub struct Operator {
	pub name: String,
	pub arity: usize,
	pub body: Sfunc,
}

macro_rules! opmap { //TODO: consolidate with function definition macros
	( $(($n:expr , $a:expr, $b:ident)),* ) => {
		{
			let mut temp_map = OpMap(BTreeMap::new());
			$(
				temp_map.add_op($n, $a, $b);
			)*
			temp_map
		}
	};
}

macro_rules! ret_num {
	( $e:expr ) => {
		vec!(Entry::Num($e))
	};
}

macro_rules! ret_pan {
	( $($e:expr),* ) => {
		vec!(Entry::Panic(format!($($e),*)))
	};
}

impl fmt::Debug for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Op {}:{}", self.name, self.arity)
	}
}

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.name, self.arity)
	}
}

impl OpMap {
	pub fn default() -> OpMap {
		opmap![("+",       2, bin_plus),
		("+",       2, bin_plus),
		("*",       2, bin_times),
		("-",       2, bin_minus),
		("/",       2, bin_divide),
		("%",       2, bin_remainder),
		("max",     2, bin_max),
		("min",     2, bin_min),
		("powf",    2, bin_powf),
		("log",     2, bin_log),
		("atan2",   2, bin_atan2),
		("hypot",   2, bin_hypot),
		("recip",   1, unary_recip),
		("floor",   1, unary_floor),
		("ceil",    1, unary_ceil),
		("round",   1, unary_round),
		("trunc",   1, unary_trunc),
		("abs",     1, unary_abs),
		("fract" ,  1, unary_fract),
		("signum",  1, unary_signum),
		("sqrt",    1, unary_sqrt),
		("exp",     1, unary_exp),
		("exp2",    1, unary_exp2),
		("ln",      1, unary_ln),
		("log2",    1, unary_log2),
		("log10",   1, unary_log10),
		("sin",     1, unary_sin),
		("cos",     1, unary_cos),
		("tan",     1, unary_tan),
		("asin",    1, unary_asin),
		("acos",    1, unary_acos),
		("atan",    1, unary_atan),
		("sinh",    1, unary_sinh),
		("cosh",    1, unary_cosh),
		("tanh",    1, unary_tanh),
		("asinh",   1, unary_asinh),
		("acosh",   1, unary_acosh),
		("atanh",   1, unary_atanh),
		("exp_m1",  1, unary_exp_m1),
		("ln_1p",   1, unary_ln_1p),
		("sum",     1, sum),
		("prod",    1, prod),
		("pop",     1, pop),
		("swap",    2, swap),
		("dup",     1, dup),
		("q",       0, quit)]
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> Values<String,Operator> {
		self.0.values()
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
			ParseType::Flt(z)   => Entry::Num(z),
			ParseType::Empty    => Entry::Id,
			ParseType::Str(s)   => match self.get(&s) {
				Some(o) => Entry::Op(o.clone()),
				None    => Entry::Panic(format!("Unknown Operator \"{}\"", s)),
			},
		}
	}
}

macro_rules! bin_op {
	( $op:tt, $n:ident ) => {
		fn $n(v: &[Entry]) -> Vec<Entry> {
			match v {
				&[Entry::Num(x), Entry::Num(y)] => ret_num!(x $op y),
				_ => ret_pan!("bad args: {:?}", v),
			}
		}
	};
}

bin_op!(+, bin_plus);
bin_op!(*, bin_times);
bin_op!(-, bin_minus);
bin_op!(/, bin_divide);
bin_op!(%, bin_remainder);

macro_rules! unary_func {
	( $op:tt, $n:ident ) => {
		fn $n(v: &[Entry]) -> Vec<Entry> {
			vec!(match v {
				&[Entry::Num(x)] => Entry::Num(x.$op()),
				_ => Entry::Panic(format!("bad args: {:?}", v)),
			})
		}
	};
}

unary_func!(recip,  unary_recip);
unary_func!(floor,  unary_floor);
unary_func!(ceil,   unary_ceil);
unary_func!(round,  unary_round);
unary_func!(trunc,  unary_trunc);
unary_func!(abs,    unary_abs);
unary_func!(fract , unary_fract);
unary_func!(signum, unary_signum);
unary_func!(sqrt,   unary_sqrt);
unary_func!(exp,    unary_exp);
unary_func!(exp2,   unary_exp2);
unary_func!(ln,     unary_ln);
unary_func!(log2,   unary_log2);
unary_func!(log10,  unary_log10);
unary_func!(sin,    unary_sin);
unary_func!(cos,    unary_cos);
unary_func!(tan,    unary_tan);
unary_func!(asin,   unary_asin);
unary_func!(acos,   unary_acos);
unary_func!(atan,   unary_atan);
unary_func!(sinh,   unary_sinh);
unary_func!(cosh,   unary_cosh);
unary_func!(tanh,   unary_tanh);
unary_func!(asinh,  unary_asinh);
unary_func!(acosh,  unary_acosh);
unary_func!(atanh,  unary_atanh);
unary_func!(exp_m1, unary_exp_m1);
unary_func!(ln_1p,  unary_ln_1p);

macro_rules! bin_func {
	( $op:tt, $n:ident ) => {
		fn $n(v: &[Entry]) -> Vec<Entry> {
			vec!(match v {
				&[Entry::Num(x), Entry::Num(y)] => Entry::Num(x.$op(y)),
				_ => Entry::Panic(format!("bad args: {:?}", v)),
			})
		}
	};
}

bin_func!(max,    bin_max);
bin_func!(min,    bin_min);
bin_func!(powf,   bin_powf);
bin_func!(log,    bin_log);
bin_func!(atan2,  bin_atan2);
bin_func!(hypot,  bin_hypot);
// max, min, powf, log, atan2, hypot, mod_euc, div_euc

macro_rules! nary_func { // TODO: get rid of name
	( $op:tt, $id:expr, $i:ident, $n:expr) => {
		fn $i(v: &[Entry]) -> Vec<Entry> {
			match v {
				&[Entry::Num(n)] =>
					vec!(Entry::Op(Operator { name: format!("{}{}", $n, n), arity: n as usize,
					body: |u: &[Entry]| {
						let mut i = u.iter();
						let mut c: f64 = match i.next() {
							Some(Entry::Num(m)) => *m,
							x => return ret_pan!("bad arg: {:?} in {:?}", x, u),
						};
						for x in i {
							if let Entry::Num(m) = x {
								c = c $op m;
							} else {
								return ret_pan!("bad arg: {:?} in {:?}", x, u);
							}
						}
						ret_num!(c)
					}})),
				_ => ret_pan!("bad args: {:?}", v),
			}
		}
	};
}

nary_func!(+, 1, sum,  "sum");
nary_func!(*, 0, prod, "prod");

// mean, median, mode, sort, neg, ..

fn quit(_v: &[Entry]) -> Vec<Entry> {
	vec!(Entry::Die)
}

fn pop(_v: &[Entry]) -> Vec<Entry> {
	vec!(Entry::Pop)
}

fn dup(v: &[Entry]) -> Vec<Entry> {
	match v {
		&[ref z] => vec!(z.clone(), z.clone()),
		_ => ret_pan!("bad args: {:?}", v),
	}
}

fn swap(v: &[Entry]) -> Vec<Entry> {
	match v {
		&[ref z, ref y] => vec!(y.clone(), z.clone()),
		_ => ret_pan!("bad args: {:?}", v),
	}
}

fn id(v: &[Entry]) -> Vec<Entry> {
	match v {
		&[ref z] => vec!(z.clone()),
		_ => ret_pan!("bad args: {:?}", v),
	}
}

