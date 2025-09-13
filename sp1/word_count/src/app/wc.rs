use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct WordCount;

pub enum Mode<'a> {
    File(&'a str),
    Text(&'a str),
}

impl<'a> Mode<'a> {
    fn to_reader<'b>(&'b self) -> Box<dyn BufRead + 'b> {
        match self {
            Mode::File(path) if *path == "-" => Box::new(std::io::stdin().lock()),
            Mode::File(path) => {
                let file = File::open(path).unwrap();
                Box::new(BufReader::new(file))
            }
            Mode::Text(text) => Box::new(text.as_bytes()),
        }
    }
}

impl WordCount {
    pub fn count(&self, mode: Mode) -> usize {
        let reader = mode.to_reader();
        self.count_from_reader(reader)
    }

    fn count_from_reader<T: BufRead>(&self, reader: T) -> usize {
        reader
            .lines()
            .map(|line| line.unwrap())
            .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<_>>())
            .collect::<BTreeSet<_>>().len()
    }
}