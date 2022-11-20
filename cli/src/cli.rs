use clap::Parser;
use std::path::PathBuf;

/// Baerscript
#[derive(Parser)]
#[clap(
	author,
	version,
	about,
	help_template = "
{author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
)]
pub struct Args {
	/// The path to the baerscript file
	#[clap(value_name = "FILE", index = 1)]
	pub path: PathBuf,

	/// Whether to show debugging information
	#[clap(short, long)]
	pub debug: bool,

	/// Whether to use ascii
	#[clap(short, long)]
	pub ascii: bool,
}
