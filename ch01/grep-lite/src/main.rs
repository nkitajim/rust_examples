use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
	name = env!("CARGO_PKG_NAME"),
	version = env!("CARGO_PKG_VERSION"),
	author = env!("CARGO_PKG_AUTHORS"),
	about = env!("CARGO_PKG_DESCRIPTION"),
	arg_required_else_help = true,
)]
struct Cli {
	#[arg(required = true)]
	pattern: String,

#[arg(default_value = "-")]
	input: String,
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
	for line_ in reader.lines() {
		let line = line_.unwrap();
		let l = line.trim_end();

		match re.find(&line) {
			Some(_) => println!("{}", l),
			None => (),
		}
	}
}

fn main() {
	let cli = Cli::parse();

	let pattern = cli.pattern;
	let re = Regex::new(&pattern).unwrap();

	let input = cli.input;

	if input == "-" {
		let stdin = io::stdin();
		let reader = stdin.lock();
		process_lines(reader, re);
	} else {
		let f = File::open(input).unwrap();
		let reader = BufReader::new(f);
		process_lines(reader, re);
	}
}
