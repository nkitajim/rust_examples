use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "temperture", about = "Change Temperture")]
#[clap(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        author = env!("CARGO_PKG_AUTHORS"),
        about = env!("CARGO_PKG_DESCRIPTION"),
        arg_required_else_help = true,
)]
pub struct Cli {
        #[arg(required = true)]
        pub temperture: f64,

        #[arg(required = true, short, value_parser = ["Celsius", "Fahrenheit", "Kelvin", "c", "f", "k"])]
        pub input_mode: String,

        #[arg(required = true, short, value_parser = ["Celsius", "Fahrenheit", "Kelvin", "c", "f", "k"])]
        pub output_mode: String,
}
