use clap::Parser;
use word_count::app::cli::Cli;
use word_count::app::wc::{Mode, WordCount};

fn main() {
    let cli = Cli::parse();

    let mode = match cli.mode.as_str() {
        "text" => Mode::Text(&cli.text),
        "file" => Mode::File(&cli.text),
        _ => panic!("Invalid mode"),
    };

    println!("{}", WordCount.count(mode));
}
