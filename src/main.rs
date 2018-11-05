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
		// draw_screen(scr_x, scr_y);
		// draw_stack(3, 2, 21, scr_y-4, &r.active());
		// if let Some(s) = r.status {
		// 	draw_status(&s, 2, scr_y-1);
		// 	r.status = None;
		// }
		// draw_prompt(scr_y);
		io::stdout().flush().unwrap();

		let mut s = String::new();
		io::stdin().read_line(&mut s)
			.expect("Failed to read line.");

		let e = match s.trim().parse::<ParseType>()
			.map(|i| OpMap::get_entry(&r.operators, i)) {
				Ok(val) => val,
				Err(_) => panic!("parse error: {}", s),
			};
		r.active().push(e);

		'proc: loop {
			match r.active().peek() {
				Some(Entry::Panic(_)) => r.status = Some(r.active().panic()),
				Some(Entry::Op(_)) => r.active().operate(),
				Some(Entry::Die) => break 'main,
				Some(Entry::Pop) => { r.active().pop(); },
				// Some(Entry::Id) => (),
				_ => break 'proc,
			}
		}
	}
	reset_screen();
	screen.flush().unwrap();
}
