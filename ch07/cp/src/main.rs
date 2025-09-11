use clap::Parser;
use cp::fs;
use cp::cli::Cli;

fn main() {
	let cli = Cli::parse();
	if cli.recursive {
		match fs::copy_recursive(&cli.source, &cli.target, cli.permission) {
			Ok(_) => (),
			Err(e) => {
				eprintln!("Cloud not copy: {e}");
				std::process::exit(1);
			}
		}
	} else {
		match fs::copy_file(&cli.source, &cli.target, cli.permission) {
			Ok(_) => (),
			Err(e) => {
				eprintln!("Cloud not copy: {e}");
				std::process::exit(1);
			}
		}
	}
}
