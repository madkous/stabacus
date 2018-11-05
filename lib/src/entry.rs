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
use std::str::FromStr;
use std::string::ParseError;

use operator::*;
use command::*;

#[derive(Clone,Debug)]
pub enum Entry {
	Num(f64),
	Panic(String),
	Cmd(Command),
	Op(Operator),
	Quote(Box<Entry>),
	Die,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Entry::Op(o)    => write!(f, "{}",  o),
			Entry::Num(n)   => write!(f, "{}",  n),
			Entry::Panic(s) => write!(f, "{}", s),
			Entry::Quote(q) => write!(f, "'{}", q),
			Entry::Cmd(_)   => write!(f, "cmd"),
			Entry::Die      => write!(f, "Die"),
			// Entry::Pop      => write!(f, "Pop"),
			// Entry::Id       => write!(f, "Id"),
		}
    }
}

#[derive(Debug)]
pub enum ParseType {
	Str(String),
	Flt(f64),
	Empty,
}

impl FromStr for ParseType {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.is_empty() {
			return Ok(ParseType::Empty)
		}
		match s.parse::<f64>() {
			Ok(n)  => Ok(ParseType::Flt(n)),
			Err(_) => Ok(ParseType::Str(s.to_string())),
		}
	}
}

