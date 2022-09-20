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

	match grid::Grid::try_from(args.path) {
		Ok(mut grid) => println!(
			"[execution completed after {} steps]",
			parser::interpret(&mut grid, args.ascii, args.debug)
		),
		Err(e) => eprintln!("error while parsing script:\n{}", e),
	}

	Ok(())
}
