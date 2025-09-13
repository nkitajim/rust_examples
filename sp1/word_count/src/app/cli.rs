use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "word_count", about = "Word Count")]
#[clap(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        author = env!("CARGO_PKG_AUTHORS"),
        about = env!("CARGO_PKG_DESCRIPTION"),
        arg_required_else_help = true,
)]
pub struct Cli {
        #[arg(required = true, value_parser = ["text", "file"])]
        pub mode: String,

        #[arg(required = true)]
        pub text: String,
}