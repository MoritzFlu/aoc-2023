// File library
use std::fs::read_to_string;

struct ConfigLine{
    data: String
}

const STRING_DIGITS: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];


impl ConfigLine {

    pub fn new(line: String) -> ConfigLine {
        ConfigLine{data: line}
    }

    fn char_to_num(&self,c: char) -> i32 {
        let num:i32 = c.to_digit(10).unwrap().try_into().unwrap();
        num
    }

    pub fn get_num(&self) -> i32 {

        println!("Starting search for: {}",self.data);

        let mut sum: i32 = 0;

        let length = self.data.chars().count();
        let c_bytes = self.data.as_bytes();


        // Start from beginning
        for i in 0..length {
            let c = c_bytes[i] as char;
            //println!("Current char at {}: {}",i,c);
            let mut digit: i32 = -1;

            // Check if c is a digit
            if c.is_numeric() {
                // Add digit to sum, times 10 since this is the larger digit
                digit = self.char_to_num(c.clone());

            } else {
                // Check if c is a string digit
                // Could be improved by building a tree from STRING_DIGITS
                for j in 0..(STRING_DIGITS.len()) {

                    // Get string to match
                    let poss = STRING_DIGITS[j];

                    // k is current comparison index in full string, starts at i
                    let mut k: usize = i;
                    let mut matched = true;

                    // Iterate over all chars in comparison string
                    for match_c in poss.chars() {
                        // Get correspondign char in full string
                        let target_c = c_bytes[k] as char;

                        // If char mismatches, stop checking this string
                        if target_c.clone() != match_c {
                            matched = false;
                            break
                        }

                        // Chat matched, compare next
                        k = k + 1;
                    }
                    if matched {
                        // Add found digit to sum
                        digit = (j+1).try_into().unwrap();
                    }
                    
                }
            }
            if digit != -1 {
                println!("Found: {}",digit);
                sum = sum + (digit*10);
                break;
            }
            

        }


        // Start from end
        for i in (0..length).rev() {
        
            let c = c_bytes[i] as char;
            //println!("Current char at {}: {}",i,c);

            let mut digit: i32 = -1;

            // Check if c is a digit
            if c.is_numeric() {
                // Add digit to sum, times 10 since this is the larger digit
                digit = self.char_to_num(c.clone());
            } else {
                // Check if c is a string digit
                // TODO: Could be improved by building a tree from STRING_DIGITS
                for j in 0..(STRING_DIGITS.len()) {

                    // Get string to match
                    let poss = STRING_DIGITS[j];

                    // k is current comparison index in full string, starts at i
                    let mut k: usize = i;
                    let mut matched = true;
                    
                    // Iterate over all chars in comparison string
                    for match_c in poss.chars().rev() {
                        
                        // Get correspondign char in full string
                        let target_c = c_bytes[k] as char;

                        // If char mismatches, stop checking this string
                        if target_c.clone() != match_c {
                            matched = false;
                            break
                        }

                        // Chat matched, compare next
                        k = k - 1;
                    }

    
                    // Add found digit to sum
                    if matched {
                        // Add found digit to sum
                        digit = (j+1).try_into().unwrap();
                    }
                }
            }

            if digit != -1 {
                println!("Found: {}",digit);
                sum = sum + digit;
                break;
            }

        }
        sum
    }
}


fn main() {
    
    let mut result = 0;

    for line in read_to_string("src/data.txt").unwrap().lines() {

        // Instantiate line object
        let obj = ConfigLine::new(line.to_string());

        let num = obj.get_num();

        println!("Number: {}",num);

        result = result + num;
    }

    println!("Result: {}",result);


}
