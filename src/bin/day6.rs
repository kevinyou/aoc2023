
use std::collections::HashSet;
use aoc2023::load_file;

static DAYSTRING: &str = "day6";

fn get_num_ways(time: u64, distance: u64) -> u64 {
    let mut count = 0;
    for time_held in 0..time {
        let speed = time_held;
        let time_traveling = time - time_held;
        let distance_traveled = speed * time_traveling;
        if distance_traveled > distance {
            count += 1;
        }
    }
    count
}

fn get_num_ways_bin(time: u64, distance: u64) -> () {
}

fn solve_part1(lines: &Vec<String>) -> u64 {
    let times = lines[0]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().expect("non-int"))
        .collect::<Vec<u64>>();

    let distances = lines[1]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().expect("non-int"))
        .collect::<Vec<u64>>();

    
    let mut ans = 1;
    for i in 0..times.len() {
        ans *= get_num_ways(times[i], distances[i]);
    }
    ans
}

fn solve_part2(lines: &Vec<String>) -> u64 {
    let time = lines[0]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .expect("failed to parse number");
    let distance = lines[1]
        .split(':')
        .collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
        .expect("failed to parse number");
    println!("{} {}", time, distance);
    get_num_ways(time, distance)
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
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part1(&load_file(&file_path)),
            288
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            71503
        );
    }
}
