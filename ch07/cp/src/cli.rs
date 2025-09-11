use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cp", about = "Rust cp clone")]
#[clap(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        author = env!("CARGO_PKG_AUTHORS"),
        about = env!("CARGO_PKG_DESCRIPTION"),
        arg_required_else_help = true,
)]
pub struct Cli {
        #[arg(required = true)]
        pub source: String,

        #[arg(required = true)]
        pub target: String,

	#[arg(short)]
	pub recursive: bool,

	#[arg(short)]
	pub permission: bool,
}
