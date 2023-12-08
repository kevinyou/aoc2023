
use std::collections::HashMap;

use aoc2023::load_file;

static DAYSTRING: &str = "day8";

struct Node {
    left: String,
    right: String,
}

fn parse_nodes(lines: &Vec<String>) -> HashMap<String, Node> {
    let mut map = HashMap::new();
    for line in lines {
        let line: Vec<&str> = line.split('=').collect();
        let label = line[0].trim().to_string();
        let pair: Vec<&str> = line[1].split(',').collect();
        let left = pair[0].chars().filter(|x| x.is_alphabetic()).collect::<String>().to_string();
        let right = pair[1].chars().filter(|x| x.is_alphabetic()).collect::<String>().to_string();

        let node = Node {
            left: left,
            right: right,
        };

        map.insert(label, node);
    }

    return map;
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let directions_line = &lines[0];
    let directions = directions_line.chars().collect::<Vec<char>>();

    let node_lines = lines
        .clone()
        .into_iter()
        .skip(1)
        .filter(|line| line.len() > 0)
        .collect::<Vec<String>>();
    
    let node_map = parse_nodes(&node_lines);
    let mut position = "AAA".to_string();
    let mut steps = 0;
    let mut direction_index = 0;
    while position != "ZZZ" {
        let node = node_map.get(&position).expect("missing node");

        let direction = directions[direction_index];
        if direction == 'L' {
            position = node.left.clone();
        } else {
            position = node.right.clone();
        }

        direction_index = (direction_index + 1) % directions.len();
        steps += 1;
    }

    steps
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
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part1(&load_file(&file_path)),
            2
        );
    }

    #[test]
    fn test_part_1_ex_2() {
        let file_path = format!("./data/{DAYSTRING}/example2.txt");
        assert_eq!(
            solve_part1(&load_file(&file_path)),
            6
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
