use console;
use lazy_static::lazy_static;
use std::io::{self, Read, Write};

use crate::grid::{Grid, GridExt, Point};
use crate::token::Token;

lazy_static! {
	static ref TERMINAL: console::Term = console::Term::stdout();
}

/// Gets a character (ascii=true) or an integer (ascii=false) from stdin
fn input<I>(stdin: &mut I, ascii: bool) -> u32
where
	I: Read,
{
	loop {
		print!("{} > ", if ascii { "character" } else { "integer" });

		// Flush stdout to print the line above, as it will only
		// flush the buffer when it comes across a LF character
		io::stdout().flush().unwrap();

		let s = {
			loop {
				let mut buf = vec![0; 1];
				stdin.read_exact(&mut buf).unwrap();

				// Ignore CR and LF keys
				if buf[0] == 13 || buf[0] == 10 {
					continue;
				}

				break String::from_utf8(buf).unwrap();
			}
		};

		if ascii {
			if let Ok(c) = s.parse::<char>() {
				break c as u32;
			} else {
				println!("character not provided, received {}", s);
			}
		} else {
			if let Ok(u) = s.parse::<u32>() {
				break u;
			} else {
				println!("integer not provided, received {}", s);
			}
		}
	}
}

/// Prints the value to stdout (as a char if ascii=true)
fn output<O>(value: u32, stdout: &mut O, ascii: bool)
where
	O: Write,
{
	if ascii {
		stdout
			.write(&format!("{}", char::from_u32(value).unwrap_or('?')).into_bytes())
			.unwrap();
	} else {
		stdout.write(&format!("{}", value).into_bytes()).unwrap();
	}

	io::stdout().flush().unwrap();
}

/// Divides the number by 2
fn divide(x: usize) -> usize {
	(x + 1) / 2 - 1
}

/// Multiplies the number by 3
fn multiply(x: usize) -> usize {
	(x + 1) * 3
}

/// Performs the collatz sequence (3x+1 if even, x / 2 if odd)
/// Note: the sequence is backwards as x is actually x - 1
fn collatz_sequence(x: usize) -> usize {
	if x % 2 == 1 {
		divide(x)
	} else {
		multiply(x)
	}
}

/// Caluclates the ((x, y), column_value) for a given point
pub fn get_point_instructions<I, O>(
	x: usize,
	y: usize,
	point: &Point,
	value: u32,
	ascii: bool,
	stdin: &mut I,
	stdout: &mut O,
) -> ((usize, usize), u32)
where
	I: Read,
	O: Write,
{
	(
		match point.token {
			Token::Multiply => (
				if value == 0 {
					multiply(x)
				} else {
					collatz_sequence(x)
				},
				y,
			),
			Token::Down => (collatz_sequence(x), y + 1),
			Token::Up => (collatz_sequence(x), y - 1),
			_ => (collatz_sequence(x), y),
		},
		match point.token {
			Token::Add => value + 1,
			Token::Subtract => value - 1,
			Token::Left => input(stdin, ascii),
			Token::Right => {
				output(value, stdout, ascii);

				value
			}
			_ => value,
		},
	)
}

/// Interprets a grid, starting from (0, 0)
pub fn interpret<I, O>(
	grid: &mut Grid,
	ascii: bool,
	debug: bool,
	stdin: &mut I,
	stdout: &mut O,
) -> u32
where
	I: Read,
	O: Write,
{
	let mut x = 0;
	let mut y = 0;
	let mut step: u32 = 0;
	let mut value: u32;
	let mut next_value: u32 = *grid.get_value().unwrap();
	let mut looping = false;

	while let Some(point) = grid.get(x, y) {
		if debug {
			TERMINAL.clear_screen().unwrap();
			TERMINAL.move_cursor_to(0, 0).unwrap();

			println!(
				"{}\n{}\n\n({}, {}) = {}",
				grid.data.iter().map(|x| x.to_string()).collect::<String>(),
				grid,
				x + 1,
				y + 1,
				point.token,
			);

			TERMINAL.read_key().ok();
		}

		step += 1;

		match point.token {
			Token::OpenSquareBracket => looping = true,
			Token::ClosedSquareBracket => looping = false,
			_ => (),
		}

		((x, y), value) = get_point_instructions(x, y, point, next_value, ascii, stdin, stdout);

		grid.set_value(value);

		if looping && grid.x < grid.columns - 1 {
			grid.x += 1;
		} else {
			grid.x = x;
			grid.y = y;
		}

		let value = grid.get_value();

		if value.is_some() {
			next_value = *value.unwrap();
		}
	}

	step
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_multiply() {
		assert_eq!(3, multiply(0));
		assert_eq!(6, multiply(1));
		assert_eq!(9, multiply(2));
	}

	#[test]
	fn test_divide() {
		assert_eq!(2, divide(5));
		assert_eq!(3, divide(8));
	}

	#[test]
	fn test_collatz() {
		assert_eq!(9, collatz_sequence(2));
		assert_eq!(1, collatz_sequence(3));
	}

	#[test]
	fn test_get_point_instructions() {
		assert_eq!(
			((1, 6), 3),
			get_point_instructions(
				3,
				6,
				&Point {
					token: Token::Multiply
				},
				3,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);

		assert_eq!(
			((2, 10), 6),
			get_point_instructions(
				5,
				9,
				&Point { token: Token::Down },
				6,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);

		assert_eq!(
			((2, 8), 6),
			get_point_instructions(
				5,
				9,
				&Point { token: Token::Up },
				6,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);

		assert_eq!(
			((2, 9), 7),
			get_point_instructions(
				5,
				9,
				&Point { token: Token::Add },
				6,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);

		assert_eq!(
			((2, 9), 5),
			get_point_instructions(
				5,
				9,
				&Point {
					token: Token::Subtract
				},
				6,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);

		assert_eq!(
			((3, 0), 5),
			get_point_instructions(
				0,
				0,
				&Point { token: Token::Null },
				5,
				false,
				&mut std::io::empty(),
				&mut std::io::sink(),
			)
		);
	}
}
