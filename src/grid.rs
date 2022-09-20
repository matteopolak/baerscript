use std::path::PathBuf;

use crate::reader;
use crate::token::Token;

#[derive(Debug)]
pub struct Point {
	pub token: Token,
}

pub struct Grid {
	pub data: Vec<u32>,
	pub x: usize,
	pub y: usize,
	rows: usize,
	columns: usize,
	tokens: Vec<Point>,
}

pub trait GridTrait {
	fn new() -> Self;
	fn set_rows(&mut self, rows: usize);
	fn set_columns(&mut self, columns: usize);
	fn push(&mut self, value: Point);
	fn get(&mut self, x: usize, y: usize) -> Option<&Point>;
	fn set_value(&mut self, value: u32);
	fn get_value(&mut self) -> Option<&u32>;
}

impl GridTrait for Grid {
	fn new() -> Self {
		Self {
			tokens: Vec::new(),
			rows: 0,
			columns: 0,
			x: 0,
			y: 0,
			data: vec![],
		}
	}

	fn set_rows(&mut self, rows: usize) {
		self.rows = rows;
	}

	fn set_columns(&mut self, columns: usize) {
		self.columns = columns;
		self.data = vec![0; columns];
	}

	fn set_value(&mut self, value: u32) {
		if let Some(d) = self.data.get_mut(self.x) {
			*d = value;
		}
	}

	fn get_value(&mut self) -> Option<&u32> {
		self.data.get(self.x)
	}

	fn push(&mut self, token: Point) {
		self.tokens.push(token);
	}

	fn get(&mut self, x: usize, y: usize) -> Option<&Point> {
		if y >= self.rows {
			return None;
		}

		if x >= self.columns {
			return Some(&Point { token: Token::NULL });
		}

		Some(
			self.tokens
				.get((x + y * self.columns) as usize)
				.unwrap_or(&Point { token: Token::NULL }),
		)
	}
}

/// PathBuf -> Grid conversion
impl std::convert::TryFrom<PathBuf> for Grid {
	type Error = anyhow::Error;

	fn try_from(path: PathBuf) -> anyhow::Result<Grid, Self::Error> {
		let lines = reader::read_lines_buffered(&path)?;
		let mut grid = Grid::new();
		let mut line_number: usize = 0;
		let mut line_count: usize = 0;
		let mut prev_line_length: usize = 0;
		let mut prev_line: String = "".to_string();

		for line in lines.into_iter() {
			let mut line_length = 0;

			line_number += 1;
			line_count += 1;

			if let Ok(line) = line {
				let chars = line.chars();

				for (i, c) in chars.enumerate() {
					if let Ok(token) = Token::try_from(c) {
						match token {
							Token::BLANKSPACE => continue,
							Token::COMMENT => break,
							token @ _ => {
								grid.push(Point { token });
								line_length += 1;
							}
						}
					} else {
						return Err(anyhow::format_err!(
							"{}\n{}^\n\ninvalid token `{}` on line {}",
							line,
							" ".repeat(i),
							c,
							line_number
						));
					}
				}

				if line_length != 0 && prev_line_length != 0 && line_length != prev_line_length {
					return Err(anyhow::format_err!(
						"{} (length of {})\n{} (length of {})\n\nlength of line {} ({}) does not match previous line ({})",
						prev_line,
						prev_line_length,
						line,
						line_length,
						line_number,
						line_length,
						prev_line_length
					));
				}

				prev_line = line;
			}

			if line_length == 0 {
				line_count -= 1;
			}

			prev_line_length = line_length;
		}

		grid.set_columns(prev_line_length);
		grid.set_rows(line_count);

		Ok(grid)
	}
}

/// Display trait for Grid (either to make it a string or to print it)
impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.tokens
				.chunks(self.columns as usize)
				.enumerate()
				.map(|(y, p)| p
					.into_iter()
					.enumerate()
					.map(|(x, p)| {
						if x == self.x && y == self.y {
							'@'
						} else {
							p.token.to_string().chars().next().unwrap()
						}
					})
					.collect::<String>())
				.intersperse("\n".to_string())
				.collect::<String>()
		)
	}
}
