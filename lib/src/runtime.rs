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

// use entry::*;
use stack::*;
use operator::*;
use std::slice::Iter;

pub struct Runtime {
	stacks: Vec<Stack>,
	active: usize,
	pub operators: OpMap,
	pub status: Option<String>,
}

impl Runtime {
	pub fn new(s: &str) -> Runtime {
		Runtime {
			stacks: vec!(Stack::new("main".to_string())),
			active: 0,
			operators: OpMap::default(),
			status: Some(s.to_string()),
		}
	}

	pub fn active(&mut self) -> &mut Stack{
		&mut self.stacks[self.active]
	}

	pub fn activate(&mut self, n: usize) {
		self.active().deactivate();
		self.active = n;
		self.active().activate();
	}

	pub fn add(&mut self, s: Stack) {
		self.stacks.push(s);
	}

	pub fn iter(&self) -> Iter<Stack> {
		self.stacks.iter()
	}
}

