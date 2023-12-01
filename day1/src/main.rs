use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut total_calibration_value = 0;

    while let Some(line) = lines.next() {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        let line = line.unwrap();
        // Trailing newline
        if line.len() == 0 {
            continue;
        }
        for c in line.chars() {
            if c.is_digit(10) {
                if first_digit == None {
                    first_digit = Some(c);
                }
                last_digit = Some(c);
            }
        }

        let calibration_value = format!("{}{}", first_digit.expect("no first digit?"), last_digit.expect("no last digit?"));
        let calibration_value: u32 = calibration_value.parse().expect("not a num");
        total_calibration_value += calibration_value;
    }

    println!("{}", total_calibration_value);
}
