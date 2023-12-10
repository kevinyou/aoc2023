
use aoc2023::load_file;

static DAYSTRING: &str = "day10";

#[allow(unused_variables)]
fn solve_part1(lines: &Vec<String>) -> u32 {
    0
}

#[allow(unused_variables)]
fn solve_part2(lines: &Vec<String>) -> u32 {
    0
}


fn main() {
    // let lines = load_from_stdin();
    let file_path = format!("./data/{DAYSTRING}/part1.txt");
    let lines = load_file(&file_path);

    let part1 = solve_part1(&lines);

    println!("Part 1: {part1}");

    let part2 = solve_part2(&lines);

    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let simple_1 = format!("./data/{DAYSTRING}/example1_simple.txt");
        let complex_1 = format!("./data/{DAYSTRING}/example1.txt");
        let ans_1 = 4;

        assert_eq!(
            solve_part1(&load_file(&simple_1)),
            ans_1
        );
        assert_eq!(
            solve_part1(&load_file(&complex_1)),
            ans_1
        );

        let simple_2 = format!("./data/{DAYSTRING}/example2_simple.txt");
        let complex_2 = format!("./data/{DAYSTRING}/example2.txt");
        let ans_2 = 8;

        assert_eq!(
            solve_part1(&load_file(&simple_2)),
            ans_2
        );
        assert_eq!(
            solve_part1(&load_file(&complex_2)),
            ans_2
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            1337
        );
    }
}
