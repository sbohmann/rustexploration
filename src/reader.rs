use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::boxed::Box;

struct Reader {
    lines: Box<Lines<BufReader<File>>>
}

impl Reader {
    fn new(br: BufReader<File>) -> Reader {
        return Reader {
            lines: Box::new(br.lines())
        };
    }

    fn read(&mut self) -> String {
        self.lines.next().unwrap().unwrap()
    }
}

pub(crate) fn run() {
    let br = BufReader::new(
        File::open("input.txt")
            .expect("Could not open file"));
    let mut reader = Reader::new(br);
    let mut s = reader.read();
    println!("{s}");
    s = reader.read();
    println!("{s}");
}
