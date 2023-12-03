
use aoc2023::load_file;

static DAYSTRING: &str = "day2";

fn solve_part1(lines: &Vec<String>) -> u32 {
    let mut total_calibration_value = 0;
    for line in lines {
        //
    }
    total_calibration_value
}

fn main() {
    // let lines = load_from_stdin();
    let file_path = format!("./data/{DAYSTRING}/part1.txt");
    let lines = load_file(&file_path);

    let part1 = solve_part1(&lines);

    println!("Part 1: {part1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part1(&load_file(&file_path)),
            8,
        );
    }
}
