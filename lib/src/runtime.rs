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

use std::slice::Iter;

use entry::*;
use stack::*;
use operator::*;
use command::*;

pub struct Runtime {
	stacks: Vec<Stack>,
	active: usize,
	pub operators: OpMap,
	pub status: Option<String>,
}

impl Runtime {
	pub fn new(s: &str) -> Runtime {
		let mut r = Runtime {
			stacks: vec!(Stack::new("main".to_string())),
			active: 0,
			operators: OpMap::default(),
			status: Some(s.to_string()),
		};
		r.activate(0);
		r
	}

	pub fn active_mut(&mut self) -> &mut Stack{
		&mut self.stacks[self.active]
	}

	pub fn active(&self) -> &Stack{
		&self.stacks[self.active]
	}

	pub fn activate(&mut self, n: usize) -> String {
		if n < self.num_stacks() {
			self.active_mut().deactivate();
			self.active = n;
			self.active_mut().activate();
			return format!("Switched to stack {}.", n)
		} else {
			format!("Could not switch to stack {}: out of bounds", n)
		}
	}

	pub fn active_ind(&mut self) -> usize {
		self.active
	}

	pub fn add(&mut self, s: String) -> String {
		self.stacks.push(Stack::new(s.clone()));
		let a = self.stacks.len();
		self.activate(a-1);
		format!("Added new stack {}:{}", a-1, s)
	}

	pub fn iter(&self) -> Iter<Stack> {
		self.stacks.iter()
	}

	pub fn num_stacks(&self) -> usize {
		self.stacks.len()
	}

	pub fn proc_cmd(&mut self) -> String {
		// peek guarantees stack is nonempty and contains Cmd on top
		if let Entry::Cmd(c) = self.active_mut().pop().unwrap() {
			match c {
				Command::Stack(n) => self.activate(n),
				Command::Add(s)   => self.add(s),
				_ => format!("Unimplemented Command {:?}", c),
			}
		} else {
			format!("Unknown error, fuck")
		}
	}
}

