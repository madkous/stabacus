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

use std::io;
use std::io::{Write};
// use std::{thread, time};

extern crate termion;
use termion::screen::*;
use termion::terminal_size;

extern crate stablib;
use stablib::entry::*;
// use stablib::stack::*;
use stablib::operator::*;
use stablib::runtime::*;
use ::drawer::*;

pub mod drawer;

fn main() {
	let mut r = Runtime::new("Welcome to Stabacus!");
	let mut screen = AlternateScreen::from(io::stdout());//.into_raw_mode().unwrap());
	let (scr_x, scr_y) = terminal_size().unwrap();

	'main: loop {
		draw_runtime(scr_x, scr_y, &mut r);
		io::stdout().flush().unwrap();

		let mut s = String::new();
		io::stdin().read_line(&mut s)
			.expect("Failed to read line.");

		let e = match s.trim().parse::<ParseType>()
			.map(|i| OpMap::get_entry(&r.operators, i)) {
				Ok(val) => val,
				Err(_) => panic!("parse error: {}", s),
			};
		r.active_mut().push(e);

		'proc: loop {
			let p = r.active().peek();
			match p { //r.active().peek() {
				// Some(Entry::Panic(_)) => r.status = Some(r.active().panic()),
				// Some(Entry::Op(_)) => r.active_mut().operate(),
				// Some(Entry::Cmd(c)) => r.proc_cmd(c),
				Some(Entry::Die) => break 'main,
				_ => break 'proc,
			}
		}
	}
	reset_screen();
	screen.flush().unwrap();
}
