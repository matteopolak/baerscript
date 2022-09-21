use clap::Parser;
use runtime::{grid, parser};

mod cli;

fn main() -> anyhow::Result<()> {
	let args = cli::Args::parse();

	if !args.path.exists() {
		eprintln!("could not find the path `{}`", args.path.to_str().unwrap());

		return Ok(());
	}

	let start = std::time::SystemTime::now();

	match grid::Grid::try_from(args.path) {
		Ok(mut grid) => println!(
			"\n\n[execution completed after {} steps in {:.2}s]",
			parser::interpret(
				&mut grid,
				args.ascii,
				args.debug,
				&mut std::io::stdin(),
				&mut std::io::stdout()
			),
			start.elapsed().unwrap().as_secs_f64()
		),
		Err(e) => eprintln!("error while parsing script:\n{}", e),
	}

	Ok(())
}
