// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
	fs,
	io::{
		self,
		Read,
		Write,
	},
	path::PathBuf,
	process,
};

use clap::Parser;
use tidier::{
	Doc,
	FormatOptions,
};

/// Format HTML or XML documents.
#[derive(Parser)]
struct Args {
	/// Use N number of spaces instead of tabs for indentation
	#[arg(short, long)]
	spaces: Option<u16>,

	/// Maximum line width
	#[arg(short, long, default_value_t = 68)]
	wrap: u32,

	/// Forcibly parse input as XML (by default it's inferred from the input extension)
	#[arg(short, long)]
	xml: bool,
	/// Print diagnostics to stderr
	#[arg(short, long)]
	verbose: bool,

	/// The input document (- for stdin)
	#[arg(default_value = "-")]
	input: PathBuf,
	/// Write output to a file (- for stdout)
	#[arg(short, long, default_value = "-")]
	out: PathBuf,
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();

	let xml = args.xml || args.input.extension().is_some_and(|e| e == "xml");
	let input = if args.input.as_os_str() == "-" {
		let mut buf = String::with_capacity(1024);
		io::stdin().read_to_string(&mut buf)?;
		buf
	} else {
		fs::read_to_string(&args.input)?
	};

	let doc = Doc::new(input, xml)?;

	let out = doc.format_bytes(
		&FormatOptions::new()
			.wrap(args.wrap)
			.tabs(args.spaces.is_none())
			.indent(args.spaces.unwrap_or(4)),
	)?;

	if args.out.as_os_str() == "-" {
		io::stdout().lock().write_all(&out)?;
	} else {
		fs::write(&args.out, &out)?;
	}

	if args.verbose && doc.has_issues() {
		eprintln!("diagnostics:");
		for d in doc.diagnostics() {
			eprintln!("{d}");
		}
	}

	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("error: {e}");
		process::exit(1);
	}
}
