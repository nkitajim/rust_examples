use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, Read};

const LINE_CHARS: usize = 16;
const BYTE_CHARS: usize = 8;

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
        file: String,
}

fn read_file(file: String) -> io::Result<Vec<u8>> {
	let f = File::open(file)?;
	let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer)?;

	Ok(buffer)
}

fn format_hexdump(b: &[u8]) -> String {
	let mut s: String = String::new();
	for (i, c) in b.iter().enumerate() {
		if i % BYTE_CHARS == 0 {
			s += " ";
		}
		s += &format!(" {c:02x}");
	}
	// add space if b.len not LINE_CHARS
	for _ in b.len()..LINE_CHARS {
		s += "   ";
	}
	if b.len() <  BYTE_CHARS {
		s += " ";
	}
	s
}

fn format_ascii(b: &[u8]) -> String {
	b
	.iter()
	.map(|b| *b as char)
	.map(|b| if b.is_ascii_graphic() {b} else {'.'})
	.collect()
}

fn hexdump(file: String) -> io::Result<()> {
	println!("{file}");

	let buffer = read_file(file)?;
	let mut count: usize = 0;

	for b in buffer.chunks(LINE_CHARS) {
		println!("{:08x}{} |{}|", count, format_hexdump(b), format_ascii(b));
		count += b.len();
	}
	println!("{count:08x}");

	Ok(())
}

fn main() {
        let cli = Cli::parse();
	match hexdump(cli.file) {
		Ok(_) => (),
		Err(e) => {
			println!("Could not hexdump. {e}");
			std::process::exit(1);
		},
	}
}
