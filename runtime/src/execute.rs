use std::io::Cursor;

use crate::grid::Grid;
use crate::parser::interpret_limited;

/// The complete, serializable result of one non-interactive execution.
#[derive(Debug, PartialEq, Eq)]
pub struct ExecutionResult {
	pub ok: bool,
	pub stdout: String,
	pub stderr: String,
	pub steps: u32,
}

/// Parses and executes source with captured I/O and a hard instruction limit.
pub fn execute(source: &str, input: &str, ascii: bool, max_steps: u32) -> ExecutionResult {
	let mut grid = match Grid::try_from(source.lines()) {
		Ok(grid) => grid,
		Err(error) => {
			return ExecutionResult {
				ok: false,
				stdout: String::new(),
				stderr: format!("error while parsing script:\n{error}"),
				steps: 0,
			};
		}
	};
	let mut stdin = Cursor::new(input.as_bytes());
	let mut stdout = Vec::new();

	match interpret_limited(&mut grid, ascii, &mut stdin, &mut stdout, max_steps) {
		Ok(steps) => ExecutionResult {
			ok: true,
			stdout: String::from_utf8_lossy(&stdout).into_owned(),
			stderr: String::new(),
			steps,
		},
		Err(error) => ExecutionResult {
			ok: false,
			stdout: String::from_utf8_lossy(&stdout).into_owned(),
			stderr: error.message,
			steps: error.steps,
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn captures_successful_output() {
		let result = execute("+v\n+v\n>v", "", false, 100);

		assert!(result.ok);
		assert_eq!(result.stdout, "2");
		assert!(result.stderr.is_empty());
		assert!(result.steps > 0);
	}

	#[test]
	fn reports_parse_errors_without_running() {
		let result = execute("?", "", false, 100);

		assert!(!result.ok);
		assert!(result.stderr.contains("invalid token `?` on line 1"));
		assert_eq!(result.steps, 0);
	}

	#[test]
	fn preserves_partial_output_on_runtime_error() {
		let result = execute(">v\n<v", "", false, 100);

		assert!(!result.ok);
		assert_eq!(result.stdout, "0");
		assert!(result.stderr.contains("stdin ended"));
		assert!(result.steps > 0);
	}

	#[test]
	fn stops_at_the_instruction_limit() {
		let result = execute(".", "", false, 5);

		assert!(!result.ok);
		assert_eq!(result.steps, 5);
		assert_eq!(result.stderr, "execution exceeded the 5 step limit");
	}
}
