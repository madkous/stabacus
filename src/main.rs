use std::io;
// use std::{thread, time};

// extern crate termion;
// use termion::screen::*;

extern crate lib;
use lib::entry::*;
use lib::stack::*;
use lib::operator::*;

fn main() {
	println!("Stack Based RPN Calculator:");

	let mut stack: Stack = Stack::new();
	let operators = OpMap::default();

	// print!("\x1b[?1049h\x1b[2j");
	// thread::sleep(time::Duration::from_millis(2000));
	// println!("Hello, World!");
	// thread::sleep(time::Duration::from_millis(2000));
	// print!("\x1b[2j\x1b[?1049l");

	loop {
		let mut s = String::new();
		io::stdin().read_line(&mut s)
			.expect("Failed to read line.");

		stack.push(match s.trim().parse::<ParseType>()
			.map(|i| OpMap::get_entry(&operators, i)) {
				Ok(val) => val,
				Err(_) => panic!("parse error on: {}", s),
			});

		// println!("peeking: {:?}", stack.peek());
		loop {
			match stack.peek() {
				Some(Entry::Panic(_)) => stack.panic(),
				Some(Entry::Op(_)) => stack.operate(),
				_ => break,
			}
		}

		println!("stack: {:?}", stack);
	}
}
