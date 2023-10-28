use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::io::prelude::BufRead;

mod labyrinth;
mod map;
mod modern;
mod reader;

#[derive(Debug)]
struct WordCounter {
    count: HashMap<String, u64>,
    path: String
}

impl WordCounter {
    fn new(path: String) -> WordCounter {
        let mut instance = WordCounter {
            count: HashMap::new(),
            path: path
        };
        instance.evaluate();
        return instance;
    }

    fn evaluate(&mut self) {
        for line in self.lines() {
            let line = line.expect("Could not read line");
            let words = line.split(" ");
            for word in words {
                if word == "" {
                    continue;
                } else {
                    self.increment(word);
                }
            }
        }
    }

    fn lines(&self) -> Lines<BufReader<File>> {
        BufReader::new(
            File::open(self.path.to_string())
                .expect("Could not open file"))
            .lines()
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.count.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(self) {
        for (key, value) in self.count.iter() {
            println!("{}: {}", key, value);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path = arguments[1].clone();
    println!("Processing file: {}", path);
    WordCounter::new(path)
        .display();
    reader::run();
    modern::modern();
    labyrinth::run();
}
