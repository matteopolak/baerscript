use console;
use lazy_static::lazy_static;
use std::fmt;
use std::io::{self, Read, Write};

use crate::grid::{Grid, GridExt, Point};
use crate::token::Token;

lazy_static! {
	static ref TERMINAL: console::Term = console::Term::stdout();
}

#[derive(Debug, PartialEq, Eq)]
pub struct InterpretError {
	pub message: String,
	pub steps: u32,
}

impl fmt::Display for InterpretError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for InterpretError {}

/// Gets a character (ascii=true) or an integer (ascii=false) from stdin
fn input<I>(stdin: &mut I, ascii: bool, prompt: bool) -> Result<u32, String>
where
	I: Read,
{
	loop {
		if prompt {
			print!("{} > ", if ascii { "character" } else { "integer" });

			// Flush stdout to print the line above, as it will only
			// flush the buffer when it comes across a LF character
			io::stdout().flush().map_err(|error| error.to_string())?;
		}

		let mut byte = [0; 1];
		stdin.read_exact(&mut byte).map_err(|_| {
			"stdin ended before an input instruction could read a value".to_string()
		})?;

		// Ignore CR and LF keys.
		if byte[0] == b'\r' || byte[0] == b'\n' {
			continue;
		}

		let parsed = if ascii {
			byte[0].is_ascii().then_some(byte[0] as u32)
		} else {
			(byte[0] as char).to_digit(10)
		};

		if let Some(value) = parsed {
			return Ok(value);
		}

		let message = if ascii {
			"ASCII character not provided".to_string()
		} else {
			format!("integer not provided, received {}", byte[0] as char)
		};
		if !prompt {
			return Err(message);
		}

		println!("{message}");
	}
}

/// Prints the value to stdout (as a char if ascii=true)
fn output<O>(value: u32, stdout: &mut O, ascii: bool) -> Result<(), String>
where
	O: Write,
{
	if ascii {
		stdout
			.write_all(&format!("{}", char::from_u32(value).unwrap_or('?')).into_bytes())
			.map_err(|error| error.to_string())?;
	} else {
		stdout
			.write_all(&format!("{}", value).into_bytes())
			.map_err(|error| error.to_string())?;
	}

	Ok(())
}

fn checked_divide(x: usize) -> Result<usize, String> {
	x.checked_add(1)
		.and_then(|value| value.checked_div(2))
		.and_then(|value| value.checked_sub(1))
		.ok_or_else(|| "instruction moved before the start of the grid".to_string())
}

fn checked_multiply(x: usize) -> Result<usize, String> {
	x.checked_add(1)
		.and_then(|value| value.checked_mul(3))
		.ok_or_else(|| "instruction position overflowed".to_string())
}

fn checked_collatz_sequence(x: usize) -> Result<usize, String> {
	if x % 2 == 1 {
		checked_divide(x)
	} else {
		checked_multiply(x)
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
	get_point_instructions_checked((x, y), point, value, ascii, true, stdin, stdout)
		.expect("BaerScript instruction failed")
}

fn get_point_instructions_checked<I, O>(
	position: (usize, usize),
	point: &Point,
	value: u32,
	ascii: bool,
	prompt: bool,
	stdin: &mut I,
	stdout: &mut O,
) -> Result<((usize, usize), u32), String>
where
	I: Read,
	O: Write,
{
	let (x, y) = position;

	Ok((
		match point.token {
			Token::Multiply => (
				if value == 0 {
					checked_multiply(x)?
				} else {
					checked_collatz_sequence(x)?
				},
				y,
			),
			Token::Down => (
				checked_collatz_sequence(x)?,
				y.checked_add(1)
					.ok_or_else(|| "instruction position overflowed".to_string())?,
			),
			Token::Up => (
				checked_collatz_sequence(x)?,
				y.checked_sub(1)
					.ok_or_else(|| "instruction moved above the grid".to_string())?,
			),
			_ => (checked_collatz_sequence(x)?, y),
		},
		match point.token {
			Token::Add => value
				.checked_add(1)
				.ok_or_else(|| "column value overflowed".to_string())?,
			Token::Subtract => value
				.checked_sub(1)
				.ok_or_else(|| "column value underflowed".to_string())?,
			Token::Left => input(stdin, ascii, prompt)?,
			Token::Right => {
				output(value, stdout, ascii)?;

				value
			}
			_ => value,
		},
	))
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
	interpret_inner(grid, ascii, debug, true, None, stdin, stdout)
		.expect("BaerScript execution failed")
}

/// Interprets a grid without terminal prompts and stops before exceeding `max_steps`.
pub fn interpret_limited<I, O>(
	grid: &mut Grid,
	ascii: bool,
	stdin: &mut I,
	stdout: &mut O,
	max_steps: u32,
) -> Result<u32, InterpretError>
where
	I: Read,
	O: Write,
{
	interpret_inner(grid, ascii, false, false, Some(max_steps), stdin, stdout)
}

fn interpret_inner<I, O>(
	grid: &mut Grid,
	ascii: bool,
	debug: bool,
	prompt: bool,
	max_steps: Option<u32>,
	stdin: &mut I,
	stdout: &mut O,
) -> Result<u32, InterpretError>
where
	I: Read,
	O: Write,
{
	let mut x = 0;
	let mut y = 0;
	let mut step: u32 = 0;
	let mut value: u32;
	let mut next_value: u32 = grid.get_value().copied().unwrap_or(0);
	let mut looping = false;

	while let Some(point) = grid.get(x, y) {
		if let Some(limit) = max_steps {
			if step >= limit {
				return Err(InterpretError {
					message: format!("execution exceeded the {limit} step limit"),
					steps: step,
				});
			}
		}

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

		// Handle loop characters (a bit hacky if it needs to be generalized to >1 function)
		// TODO: Refactor into a context manager
		match point.token {
			Token::OpenSquareBracket => looping = true,
			Token::ClosedSquareBracket => looping = false,
			_ => (),
		}

		((x, y), value) =
			get_point_instructions_checked((x, y), point, next_value, ascii, prompt, stdin, stdout)
				.map_err(|message| InterpretError {
					message,
					steps: step,
				})?;
		step += 1;

		grid.set_value(value);

		if looping && grid.x < grid.columns - 1 {
			grid.x += 1;
		} else {
			grid.x = x;
			grid.y = y;
		}

		if let Some(value) = grid.get_value() {
			next_value = *value;
		}
	}

	Ok(step)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_multiply() {
		assert_eq!(Ok(3), checked_multiply(0));
		assert_eq!(Ok(6), checked_multiply(1));
		assert_eq!(Ok(9), checked_multiply(2));
	}

	#[test]
	fn test_divide() {
		assert_eq!(Ok(2), checked_divide(5));
		assert_eq!(Ok(3), checked_divide(8));
	}

	#[test]
	fn test_collatz() {
		assert_eq!(Ok(9), checked_collatz_sequence(2));
		assert_eq!(Ok(1), checked_collatz_sequence(3));
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
