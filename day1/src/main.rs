use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut total_calibration_value = 0;

    while let Some(line) = lines.next() {

        let line = line.unwrap();
        // Trailing newline
        if line.len() == 0 {
            continue;
        }

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

        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        'outer: for i in 0..line.len() {
            let substr: String = line.chars().take(i + 1).collect();
            let c = substr.chars().last().expect("Getting last char failed");

            if c.is_digit(10) {
                first_digit = c.to_digit(10);
                break 'outer;
            }

            for (word_index, digit_word) in digit_words.iter().enumerate() {
                if substr.contains(digit_word) {
                    first_digit = Some(word_index as u32 + 1);
                    break 'outer;
                }
            }
        }

        'outer: for i in (0..line.len()).rev() {
            let substr: String = line.chars().skip(i).collect();
            let c = substr.chars().nth(0).expect("Getting first char failed");

            if c.is_digit(10) {
                last_digit = c.to_digit(10);
                break 'outer;
            }

            for (word_index, digit_word) in digit_words.iter().enumerate() {
                if substr.contains(digit_word) {
                    last_digit = Some(word_index as u32 + 1);
                    break 'outer;
                }
            }
        }

        let first_digit = first_digit.expect("First digit Not found");
        let last_digit = last_digit.expect("Last digit not found");
        let calibration_value = format!("{}{}", first_digit, last_digit);
        let calibration_value: u32 = calibration_value.parse().expect("not a num");
        total_calibration_value += calibration_value;
    }

    println!("{}", total_calibration_value);
}
