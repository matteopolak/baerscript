use console;
use std::io::{self, Write};

use crate::grid::{Grid, GridTrait, Point};
use crate::token::Token;

lazy_static::lazy_static! {
	static ref TERMINAL: console::Term = console::Term::stdout();
}

/// Gets a character (ascii=true) or an integer (ascii=false) from stdin
fn input(ascii: bool) -> u32 {
	loop {
		print!("{} > ", if ascii { "character" } else { "integer" });

		io::stdout().flush().unwrap();

		let s = TERMINAL.read_line().unwrap();

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
fn output(value: u32, ascii: bool) {
	if ascii {
		print!("{}", char::from_u32(value).unwrap_or('?'));
	} else {
		print!("{}", value);
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

/// Performs the collatz conjecture (3x+1)
fn collatz_conjecture(x: usize) -> usize {
	if x % 2 == 1 {
		divide(x)
	} else {
		multiply(x)
	}
}

/// Caluclates the ((x, y), column_value) for a given point
pub fn get_point_instructions(
	x: usize,
	y: usize,
	point: &Point,
	value: u32,
	ascii: bool,
) -> ((usize, usize), u32) {
	(
		match point.token {
			Token::MULTIPLY => (
				if value == 0 {
					multiply(x)
				} else {
					collatz_conjecture(x)
				},
				y,
			),
			Token::DOWN => (collatz_conjecture(x), y + 1),
			Token::UP => (collatz_conjecture(x), y - 1),
			_ => (collatz_conjecture(x), y),
		},
		match point.token {
			Token::ADD => value + 1,
			Token::SUBTRACT => value - 1,
			Token::LEFT => input(ascii),
			Token::RIGHT => {
				output(value, ascii);

				value
			}
			_ => value,
		},
	)
}

/// Interprets a grid, starting from (0, 0)
pub fn interpret(grid: &mut Grid, ascii: bool, debug: bool) -> u32 {
	let mut x = 0;
	let mut y = 0;
	let mut step: u32 = 0;
	let mut value: u32;
	let mut next_value: u32 = *grid.get_value().unwrap();

	while let Some(point) = grid.get(x, y) {
		if debug {
			TERMINAL.clear_screen().unwrap();
			TERMINAL.move_cursor_to(0, 0).unwrap();

			println!(
				"{}\n{}\n\n({}, {}) = {:?}",
				&grid.data.iter().map(|x| x.to_string()).collect::<String>(),
				&grid,
				x + 1,
				y + 1,
				point.token,
			);

			TERMINAL.read_key().ok();
		}

		step += 1;

		((x, y), value) = get_point_instructions(x, y, point, next_value, ascii);

		grid.set_value(value);
		grid.x = x;
		grid.y = y;

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
		assert_eq!(multiply(0), 3);
		assert_eq!(multiply(1), 6);
		assert_eq!(multiply(2), 9);
	}

	#[test]
	fn test_divide() {
		assert_eq!(divide(5), 2);
		assert_eq!(divide(8), 3);
	}

	#[test]
	fn test_collatz() {
		assert_eq!(collatz_conjecture(2), 9);
		assert_eq!(collatz_conjecture(3), 1);
	}

	#[test]
	fn test_get_point_instructions() {
		assert_eq!(
			get_point_instructions(
				3,
				6,
				&Point {
					token: Token::MULTIPLY
				},
				3,
				false
			),
			((1, 6), 3)
		);

		assert_eq!(
			get_point_instructions(5, 9, &Point { token: Token::DOWN }, 6, false),
			((2, 10), 6)
		);

		assert_eq!(
			get_point_instructions(5, 9, &Point { token: Token::UP }, 6, false),
			((2, 8), 6)
		);

		assert_eq!(
			get_point_instructions(5, 9, &Point { token: Token::ADD }, 6, false),
			((2, 9), 7)
		);

		assert_eq!(
			get_point_instructions(
				5,
				9,
				&Point {
					token: Token::SUBTRACT
				},
				6,
				false
			),
			((2, 9), 5)
		);

		assert_eq!(
			get_point_instructions(0, 0, &Point { token: Token::NULL }, 5, false),
			((3, 0), 5)
		);
	}
}
