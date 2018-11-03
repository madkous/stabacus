

extern crate termion;
// use termion::raw::IntoRawMode;
use termion::{cursor, clear};

extern crate lib;
use lib::entry::*;
use lib::stack::*;
use lib::operator::*;
// unicode box drawing characters:
//        0 1 2 3 4 5 6 7 8 9 A B C D E F
// u+250x ─ ━ │ ┃ ┄ ┅ ┆ ┇ ┈ ┉ ┊ ┋ ┌ ┍ ┎ ┏
// u+251x ┐ ┑ ┒ ┓ └ ┕ ┖ ┗ ┘ ┙ ┚ ┛ ├ ┝ ┞ ┟
// u+252x ┠ ┡ ┢ ┣ ┤ ┥ ┦ ┧ ┨ ┩ ┪ ┫ ┬ ┭ ┮ ┯
// u+253x ┰ ┱ ┲ ┳ ┴ ┵ ┶ ┷ ┸ ┹ ┺ ┻ ┼ ┽ ┾ ┿
// u+254x ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊ ╋ ╌ ╍ ╎ ╏
// u+255x ═ ║ ╒ ╓ ╔ ╕ ╖ ╗ ╘ ╙ ╚ ╛ ╜ ╝ ╞ ╟
// u+256x ╠ ╡ ╢ ╣ ╤ ╥ ╦ ╧ ╨ ╩ ╪ ╫ ╬ ╭ ╮ ╯
// u+257x ╰ ╱ ╲ ╳ ╴ ╵ ╶ ╷ ╸ ╹ ╺ ╻ ╼ ╽ ╾ ╿

//  ┌─┐ ┏━┓ ╔═╗
//  │ │ ┃ ┃ ║ ║
//  └─┘ ┗━┛ ╚═╝

pub struct BoxChars {
	tl_cor: &'static str,
	tr_cor: &'static str,
	bl_cor: &'static str,
	br_cor: &'static str,
	h_bar:  &'static str,
	v_bar:  &'static str,
}

macro_rules! boxchars {
	( $id:ident, $tl:expr, $tr:expr, $bl:expr, $br:expr, $h:expr, $v:expr ) => {
		pub const $id: BoxChars = BoxChars{
			tl_cor: $tl,
			tr_cor: $tr,
			bl_cor: $bl,
			br_cor: $br,
			h_bar:  $h,
			v_bar:  $v,
		};
	};
}

boxchars!(REG, "┌", "┐", "└", "┘", "─", "│");
// boxchars!(BLD, "┏", "┓", "┗", "┛", "━", "┃");
boxchars!(DUB, "╔", "╗", "╚", "╝", "═", "║");

pub fn draw_box(x: u16, y: u16, w: u16, h: u16, b: BoxChars) {
	print!("{go}{tl}{h}{tr}",
		   go = cursor::Goto(x,y),
		   tl = b.tl_cor, tr = b.tr_cor,
		   h = b.h_bar.repeat((w - 2) as usize));
	for i in y+1..y+h {
		print!("{go1}{v}{go2}{v}",
			   v = b.v_bar,
			   go1 = cursor::Goto(x,i),
			   go2 = cursor::Goto(x+w-1,i));
	}
	print!("{go}{bl}{h}{br}",
		   go = cursor::Goto(x,y+h),
		   bl = b.bl_cor, br = b.br_cor,
		   h = b.h_bar.repeat((w - 2) as usize));
}

// WARNING: expects nonzero w
pub fn draw_title(x: u16, y: u16, w: u16, s: &String) {
	if s.len() as u16 <= w {
		let d = w - s.len() as u16;
		print!("{go}{st}",
			   go = cursor::Goto(x+(d/2), y),
			   st = s);
	} else {
		let s = &s[0..(w-1) as usize];
		print!("{go}{st}…",
			   go = cursor::Goto(x, y),
			   st = s);
	}
}

pub fn draw_stack(x: u16, y: u16, w: u16, h: u16, s: &Stack) {
	draw_box(x, y+1, w, h-1, REG);
	draw_title(x+1, y, w-2, s.name());
	let y = y+2;
	let h = h-3;
	for (i, z) in s.iter().rev().enumerate() {
		if let Entry::Int(n) = z {
			print!("{0}{1:02}:{2:.>3$}",
				   cursor::Goto(x+2, y+h-(i as u16)),
				   i, n, w as usize - 7);
			if i as u16 >= h {
				break;
			}
		}
	}
	print!("{}", cursor::Goto(1, 1));
}

pub fn draw_status(s: &str, x: u16, y: u16) {
	print!("{}{}", cursor::Goto(x, y), s);
}

pub fn reset_screen() {
	print!("{}{}", clear::All, cursor::Goto(1, 1));
}

pub fn draw_prompt(y: u16) {
	print!("{}> ", cursor::Goto(1, y-2));
}
