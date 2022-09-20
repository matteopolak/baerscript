#![feature(iter_intersperse)]

use clap::Parser;

mod cli;
mod grid;
mod parser;
mod reader;
mod token;

fn main() -> anyhow::Result<()> {
	let args = cli::Args::parse();

	if !args.path.exists() {
		eprintln!("Could not find the path {}", args.path.to_str().unwrap());

		return Ok(());
	}

	if let Ok(mut grid) = grid::Grid::try_from(args.path) {
		println!(
			"Completed execution after {} steps",
			parser::interpret(&mut grid, args.ascii, args.debug)
		);
	} else {
		eprintln!("Could not parse the script, is it formatted correctly?");
	}

	Ok(())
}
