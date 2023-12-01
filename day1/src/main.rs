use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut total_calibration_value = 0;

    while let Some(line) = lines.next() {

        let mut line = line.unwrap();
        // Trailing newline
        if line.len() == 0 {
            continue;
        }

        // Replace all instances of word with digit
        let digit_words = [
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
        ];

        for (index, word) in digit_words.iter().enumerate() {
            let val = (index + 1).to_string();
            line = line.replace(word, &val);
        }

        let first_digit = line.find(|c: char| c.is_digit(10)).expect("no first digit?");
        let first_digit = line
            .chars()
            .nth(first_digit)
            .expect("Index from find not found");

        let last_digit = line.rfind(|c: char| c.is_digit(10)).expect("no second digit?");
        let last_digit = line
            .chars()
            .nth(last_digit)
            .expect("Index from find not found");

        let calibration_value = format!("{}{}", first_digit, last_digit);
        let calibration_value: u32 = calibration_value.parse().expect("not a num");
        total_calibration_value += calibration_value;
    println!("{}", calibration_value);
    }

    // println!("{}", total_calibration_value);
}
