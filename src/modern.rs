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

fn WordCount(reader: BufReader<File>) {
    let count: HashMap::new();

    evaluate();

    fn evaluate() {
        for line in reader.lines() {
            let line = line.expect("Could not read line");
            let words = line.split(" ");
            for word in words {
                if word == "" {
                    continue;
                } else {
                    increment(word);
                }
            }
        }
    }

    fn increment(word: &str) {
        let key = word.to_string();
        let count = self.count.entry(key).or_insert(0);
        *count += 1;
    }

    fn display() {
        for (key, value) in count.iter() {
            println!("{}: {}", key, value);
        }
    }
}

fn run() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    WordCounter::new(reader)
        .display();
}
