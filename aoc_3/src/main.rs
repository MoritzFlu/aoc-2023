use regex::Regex;
use std::fs::read_to_string;


struct Analyzer {
    lines: Vec<Vec<u8>>
}

impl Analyzer {
    
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
            // x number of lines reached
            lines.remove(0);
        }

        // add new line to vector
        lines.push(bytes.to_vec());

        if i == 3 {

            let obj = 
        }

    }


}
