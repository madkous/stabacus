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

extern crate lib;
use lib::entry::*;
use lib::stack::*;
use lib::operator::*;
use ::drawer::*;

pub mod drawer;

fn main() {
	let mut stack: Stack = Stack::new("main".to_string());
	let operators = OpMap::default();
	{
		let mut screen = AlternateScreen::from(io::stdout());//.into_raw_mode().unwrap());
		let (scr_x, scr_y) = terminal_size().unwrap();
		let mut status: Option<String> = Some("Welcome to RPN calc".to_string());

		'main: loop {
			reset_screen();
			draw_box(1, 1, scr_x, scr_y-5, DUB);
			draw_stack(3, 3, 20, 20, &stack);
			if let Some(s) = status {
				draw_status(&s, 2, scr_y-1);
				status = None;
			}
			draw_prompt(scr_y);

			io::stdout().flush().unwrap();

			let mut s = String::new();
			io::stdin().read_line(&mut s)
				.expect("Failed to read line.");

			// if i < 10 {
			// 	thread::sleep(time::Duration::from_secs(1));
			// 	stack.push(Entry::Int(i));
			// 	i += 1;
			// } else {
			// 	thread::sleep(time::Duration::from_secs(1));
			// 	stack.push(Entry::Die);
			// }

			stack.push(match s.trim().parse::<ParseType>()
					   .map(|i| OpMap::get_entry(&operators, i)) {
						   Ok(val) => val,
						   Err(_) => panic!("parse error: {}", s),
					   });

			'proc: loop {
				match stack.peek() {
					Some(Entry::Panic(_)) => status = Some(stack.panic()),
					Some(Entry::Op(_)) => stack.operate(),
					Some(Entry::Die) => break 'main,
					Some(Entry::Pop) => { stack.pop(); },
					_ => break 'proc,
				}
			}
		}
		reset_screen();
		screen.flush().unwrap();
	}
}
