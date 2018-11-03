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

use entry::*;
use std::slice::Iter;

#[derive(Debug)]
pub struct Stack(Vec<Entry>, String);

impl Stack {
	//reimplement new, len, push, pop, split_off
	pub fn new(s: String) -> Stack {
		Stack(Vec::new(), s)
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn push(&mut self, v: Entry) {
		self.0.push(v)
	}

	pub fn pop(&mut self) -> Option<Entry> {
		self.0.pop()
	}

	pub fn as_slice(&self) -> &[Entry] {
		self.0.as_slice()
	}

	pub fn peek(&self) -> Option<&Entry> {
		self.0.as_slice().last()
	}

	pub fn iter(&self) -> Iter<Entry> {
		self.0.iter()
	}

	pub fn name(&self) -> &String {
		&self.1
	}

	pub fn pop_slice(&mut self, depth: usize) -> Stack {
		assert!(depth <= self.len(), "`depth` greater than stack height");
		let height = self.len() - depth;
		Stack(self.0.split_off(height), "args".to_string())
	}

	pub fn operate(&mut self) {
		let r = match self.pop() {
			Some(Entry::Op(op)) =>
				if self.len() < op.arity{
					Entry::Panic(format!("cannot apply: {} has arity {}, stack has {} elements",
										 op.name, op.arity, self.len()))
				} else {
					let args = self.pop_slice(op.arity);
					(op.body)(args.as_slice())
				},
			Some(e) =>
				Entry::Panic(format!("tried to operate with non-operator entry: {:?}",
									 e)),
			None =>
				Entry::Panic("tried to operate with empty stack".to_string()),
		};
		self.push(r);
	}

	pub fn panic(&mut self) -> String {
		format!("Panicked on: {:?}", self.pop())
	}
}
