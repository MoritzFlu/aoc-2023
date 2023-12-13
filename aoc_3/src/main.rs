use regex::Regex;
use std::fs::read_to_string;


struct Analyzer {
    lines: Vec<Vec<u8>>
}

impl Analyzer {
    const look_pattern: [[i8; 2];8] = [
        [0,-1],// upper left 
        [0,0], // upper mid
        [0,1], // upper reight
        [2,-1], // lower left
        [2,0],  // lower mid
        [2,1],  // lower right
        [0,-1], // left 
        [0,1]   // right
        ];

    pub fn new(lines: Vec<Vec<u8>>) -> Analyzer {
        Analyzer { lines: lines }
    }

    pub fn get_sum(&self) -> i32 {

        let num: String;

        let mut found_symbol = false;

        for i in 0..self.lines[1].len() {

            let c = self.lines[1][i] as char;

            if !found_symbol {
                for patt in Analyzer::look_pattern {
                    let j = patt[0] as usize;
                    let k = (i as i8) + patt[1];
                    if  k > 0 {
                        // select adjacent char
                        let check_symbol = self.lines[j][k as usize] as char;

                        if Analyzer::is_symbol(check_symbol) {
                            found_symbol = true;
                        }
                    }
                    
                }
            }
        }
        0
    }

    fn is_symbol(input: char) -> bool {
        if input.is_digit(10) {return false}
        if input == '.' {return false}
        true
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
