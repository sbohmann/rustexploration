// fn main() {
//     let name = String { vec: "x" };
//     name += "yes";
//     println!("Hello, world! {name}");
// }

// word_counter.rs

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::BufRead;

#[derive(Debug)]
struct WordCounter {
    count: HashMap<String, u64>,
    reader: BufReader<File>
}

impl WordCounter {
    fn new(reader: BufReader<File>) -> WordCounter {
        let mut instance = WordCounter {
            count: HashMap::new(),
            reader
        };
        instance.evaluate();
        return instance;
    }

    fn evaluate(&mut self) {
        for line in self.reader.lines() {
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
    let filename = &arguments[1];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    WordCounter::new(reader)
        .display();
}
