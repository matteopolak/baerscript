use clap::Parser;
use std::path::PathBuf;

/// Baerscript
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Matthew Polak")]
pub struct Args {
	/// The path to the baerscript file
	#[clap(value_name = "FILE", index = 1)]
	pub path: PathBuf,

	/// Whether to show debugging information
	#[clap(short, long, takes_value = false)]
	pub debug: bool,

	/// Whether to use ascii
	#[clap(short, long, takes_value = false)]
	pub ascii: bool,
}
