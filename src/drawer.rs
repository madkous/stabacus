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

extern crate termion;
// use termion::raw::IntoRawMode;
use termion::{cursor, clear};//, style};

extern crate stablib;
use stablib::entry::*;
use stablib::stack::*;
use stablib::runtime::*;
use stablib::operator::*;

const STACK_MAX_W: u16 = 45;
const STACK_MIN_W: u16 = 25;
const OPS_W: u16 = 15;

struct BoxChars {
	tl_cor: &'static str, tr_cor: &'static str,
	bl_cor: &'static str, br_cor: &'static str,
	h_bar:  &'static str, v_bar:  &'static str,
	l_head: &'static str, r_head: &'static str,
	t_head: &'static str, b_head: &'static str,
}

macro_rules! boxchars {
	( $id:ident, $tl:expr, $tr:expr, $bl:expr, $br:expr, $h:expr, $v:expr,
	  $lh:expr, $rh:expr, $th:expr, $bh:expr ) => {
		const $id: BoxChars = BoxChars{
			tl_cor: $tl, tr_cor: $tr,
			bl_cor: $bl, br_cor: $br,
			h_bar:  $h, v_bar:  $v,
			l_head: $lh, r_head: $rh,
			t_head: $th, b_head: $bh,
		};
	};
}
// collection of box drawing characters from u+25xx
// ┌───┐ ╔═════════╗ ┏━━━━━━━━━┓ ╒╕╓╖┍┑┎┒╭╮╆╅ ┊┋ ╷┟┧┢┪╉╆╅╁┾┿┽
// │├┼┬│ ║╠╬╦╟╫╥╞╪╤║ ┃┣╋┳┠╁┰┝╈┯┃ ╘╛╙╜┕┙┖┚╰╯╄╃ ┆┇ ╽┞┦┡┩╂╄╃╀
// │┴┼┤│ ║╩╬╣╨╫╢╧╪╡║ ┃┻╋┫┸╀┨┷╇┥┃ ┲┱┮┭╶╼╸╺╾╴╎╏ ╵╹ ╻    ╊
// └───┘ ╚═════════╝ ┗━━━━━━━━━┛ ┺┹┶┵┄┈╌┅┉╍╱╲╳   ╿
boxchars!(REG, "┌", "┐", "└", "┘", "─", "│", "┤", "├", "┴", "┬");
boxchars!(BLD, "┏", "┓", "┗", "┛", "━", "┃", "┥", "┝", "┸", "┰");
boxchars!(DUB, "╔", "╗", "╚", "╝", "═", "║", "╡", "╞", "╨", "╥");

pub fn draw_runtime(w: u16, h: u16, r: &mut Runtime) {
	draw_screen(w, h);
	draw_stacks(w - (OPS_W + 5), h, r);
	if let Some(ref s) = r.status {
		draw_status(&s, 2, h-1);
	}
	r.status = None; // TODO: can this be moved into the conditional?
	draw_ops(w-(OPS_W + 2), 2, OPS_W, h-5, &r.operators);
	draw_prompt(h);
}

pub fn reset_screen() {
	print!("{}{}", clear::All, cursor::Goto(1, 1));
}

fn draw_box(x: u16, y: u16, w: u16, h: u16, b: &BoxChars) {
	print!("{0}{1}{3}{2}", cursor::Goto(x,y),
	b.tl_cor, b.tr_cor, b.h_bar.repeat((w - 2) as usize));
	for i in y+1..y+h {
		print!("{1}{0}{2}{0}", b.v_bar,
			   cursor::Goto(x,i), cursor::Goto(x+w-1,i));
	}
	print!("{0}{1}{3}{2}", cursor::Goto(x,y+h),
	b.bl_cor, b.br_cor, b.h_bar.repeat((w - 2) as usize));
	// style::Blink, style::Reset );
}

fn draw_ops(x: u16, y: u16, w: u16, h: u16, ops: &OpMap) {
	let l = h.min(ops.len() as u16 + 1);
	draw_box(x, y, w, l, &REG);
	draw_title(x+2, y, &REG, "Operators");
	for (i, o) in ops.iter().enumerate() {
		print!("{0}{1}",
			   cursor::Goto(x+2, y+l-(i as u16+1)),
			   o);
		if i as u16 >= l-2 {
			break;
		}
	}
}

fn draw_prompt(y: u16) {
	print!("{}> ", cursor::Goto(1, y));
}

fn draw_screen(w: u16, h: u16) {
	reset_screen();
	draw_box(1, 1, w, h-3, &DUB);
	draw_title(5, 1, &DUB, "Stabacus");
}

// WARNING: minimum w is 5
fn draw_stack(x: u16, y: u16, w: u16, h: u16, s: &Stack) {
	let b = if s.is_active() { &BLD } else { &REG };
	draw_box(x, y, w, h-1, b);
	let l = s.name().len() as u16;
	let o = if l <= w-4 { (w - (l+2)) / 2 } else { 1 };
	draw_title(x+o, y, b, &fit_name(w-4, s.name()));
	let y = y+1;
	let h = h-3;
	for (i, z) in s.iter().rev().enumerate() {
		// TODO: most of this logic should be in entry
		if let Entry::Num(n) = z {
			print!("{0}{1:02}:{2:.>3$}",
				   cursor::Goto(x+2, y+h-(i as u16)),
				   i, n, w as usize - 7);
		} else {
			print!("{0}{1:02}:{2:?}",
				   cursor::Goto(x+2, y+h-(i as u16)),
				   i, z);
		}
		if i as u16 >= h {
			break;
		}
	}
}

fn draw_stacks(w: u16, h: u16, r: &mut Runtime) {
	// maximum number of stacks given width
	let n = (w / STACK_MIN_W) as usize;
	let z = r.num_stacks();
	// width of each stack given n
	let e = (w / n.min(z) as u16).min(STACK_MAX_W); // works
	let k; // start index
	let l; // stop index
	if (n as usize) >= z {
		k = 0 as usize;
		l = z as usize;
	} else {
		let a = r.active_ind();
		if a < n / 2 {
			k = 0;
			l = n;
		} else if a >= z - n / 2 {
			k = z - n;
			l = z;
		} else {
			k = a - n / 2;
			l = a + (n - (n / 2));
		}
	}

	for (i, s) in r.iter().enumerate() {
		if i < k {
			continue;
		}
		if i >= l {
			break;
		}
		draw_stack(3 + (e * (i-k) as u16), 2, e, h-4, &s);
	}
}

fn draw_status(s: &str, x: u16, y: u16) {
	print!("{}{}", cursor::Goto(x, y), s);
}

fn draw_title(x: u16, y: u16, b: &BoxChars, s: &str) {
	print!("{0}{1}{3}{2}", cursor::Goto(x,y), b.l_head, b.r_head, s);
}

// WARNING: expects nonzero w
fn fit_name(w: u16, s: &str) -> String{
	if s.len() as u16 <= w {
		s.to_string()
			// let d = w - s.len() as u16;
			// print!("{}{}", cursor::Goto(x+(d/2), y), s);
	} else {
		format!("{}…", &s[0..(w-1) as usize])
			// print!("{}{}…", cursor::Goto(x, y), &s[0..(w-1) as usize]);
	}
}
