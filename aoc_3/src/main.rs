use regex::Regex;
use std::fs::read_to_string;


struct Analyzer {
    lines: Vec<Vec<u8>>
}

impl Analyzer {
    const SYMBOLS : [&str; 2] = ["*", "&"];

    pub fn new(lines: Vec<Vec<u8>>) -> Analyzer {
        Analyzer { lines: lines }
    }

    pub fn get_sum(&self) -> i32 {
        0
    }
}

fn main() {
    let mut lines: Vec<Vec<u8>> = Vec::new();

    let mut i = 0;

    for line in read_to_string("src/data.txt").unwrap().lines() {
        let bytes = line.as_bytes();

        if i < 3 {
            // counts numer of rows i;n lines
            i = i + 1;
        } else {
            // 4 lines stored, remove oldest
            lines.remove(0);
        }

        // add new line to vector
        lines.push(bytes.to_vec());

        // There are already 3 lines stored, begin object creation
        if i == 3 {
            let obj = Analyzer::new(lines.clone());
        }

    }


}
