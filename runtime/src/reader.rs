use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use std::path::PathBuf;

/// Reads a file and iterates over each line
pub fn read_lines_buffered(path: &PathBuf) -> Result<Lines<BufReader<File>>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	Ok(reader.lines())
}
