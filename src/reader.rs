use std::fs::File;
use std::io::{BufRead, BufReader};

struct Reader {
    br: BufReader<File>
}

impl Reader {
    fn new(br: BufReader<File>) -> Reader {
        return Reader {
            br
        };
    }

    // fn read(&mut self) {
    //     let lines = self.br.lines();
    // }
}

pub(crate) fn run() {
    let br = BufReader::new(
        File::open("input.txt")
            .expect("Could not open file"));
    let mut reader = Reader::new(br);
    // reader.read();
}
