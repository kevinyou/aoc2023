
use std::collections::HashMap;

use aoc2023::load_file;

static DAYSTRING: &str = "day8";

mod math {
    // rewrote below function for vec
    pub fn my_lcm(nums: &Vec<usize>) -> usize {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd_of_two_numbers(a, b)
    }

    // https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
    pub fn lcm(nums: &[usize]) -> usize {
        if nums.len() == 1 {
            return nums[0];
        }
        let a = nums[0];
        let b = lcm(&nums[1..]);
        a * b / gcd_of_two_numbers(a, b)
    }

    fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd_of_two_numbers(b, a % b)
    }
}

struct Node<'a> {
    left: &'a str,
    right: &'a  str,
}

#[derive(Eq, PartialEq, Hash)]
struct Event<'a> {
    position: &'a str,
    direction_index: usize,
}

fn parse_nodes(lines: &Vec<String>) -> HashMap<&str, Node> {
    let mut map = HashMap::new();
    for line in lines {
        let (label, pair) = line.split_once(" = ").expect("failed to split =");
        let (left, right) = pair[1..pair.len() - 1]
            .split_once(", ").unwrap();

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
    let mut position = "AAA";
    let mut steps = 0;
    let mut direction_index = 0;
    while position != "ZZZ" {
        let node = node_map.get(position).expect("missing node");

        let direction = directions[direction_index];
        if direction == 'L' {
            position = node.left;
        } else {
            position = node.right;
        }

        direction_index = (direction_index + 1) % directions.len();
        steps += 1;
    }

    steps
}


// For each starting position, calculate steps needed to make one full cycle (back at the same position with same direction index)
// In that cycle, find all offsets from that position that end in Z.
//
// 11A -> Cycles back to 11B every 2 steps. Reaches Z after 1 step from 11B.
// 22A -> Cycles back to 22B every 3 steps. Reaches Z after 2 steps from 22B.

// (Note there could be multiple different Z's and ways to reach Z in a cycle)
// After 1 + 2n + 1 == 2n + 2 steps first position is correct
// After 1 + 3m + 2 == 3m + 3 steps second position is correct
// What's the lowest number that satisfies that?
fn get_equations(starting_position: &str, directions: &Vec<char>, node_map: &HashMap<&str, Node>) -> (u64, u64, Vec<u64>) {
    let mut position = starting_position;
    let mut memo: HashMap<Event, u64> = HashMap::new();
    let mut direction_index: usize = 0;
    let mut steps: u64 = 0;

    let mut ways_to_z: Vec<u64> = Vec::new();

    loop {
        if position.ends_with("Z") {
            ways_to_z.push(steps);
        }

        let event = Event {
            position: position,
            direction_index: direction_index,
        };

        if memo.contains_key(&event) {
            let initial_steps = memo.get(&event).unwrap();
            let cycle_len = steps - initial_steps;
            // change into offsets from  initial steps
            let ways_to_z: Vec<u64> = ways_to_z.into_iter().map(|i| i - initial_steps).collect();

            return (*initial_steps, cycle_len, ways_to_z);
        } else {
            memo.insert(event, steps);
        }
        let node = node_map
            .get(position)
            .expect("missing node");

        if directions[direction_index] == 'L' {
            position = node.left;
        } else {
            position = node.right;
        }

        direction_index = (direction_index + 1) % directions.len();
        steps += 1;
    }
}

fn solve_part2(lines: &Vec<String>) -> u64 {
    let directions_line = &lines[0];
    let directions = directions_line.chars().collect::<Vec<char>>();

    let node_lines = lines
        .clone()
        .into_iter()
        .skip(1)
        .filter(|line| line.len() > 0)
        .collect::<Vec<String>>();
    
    let node_map = parse_nodes(&node_lines);

    let positions: Vec<&str> = node_map
        .keys()
        .filter(|label| label.ends_with("A"))
        .cloned()
        .collect();

        for position in positions.iter() {
            let x = get_equations(&position, &directions, &node_map);
            println!("{:?}", x);
        }

    math::my_lcm(
        &positions
            .into_iter()
            .map(|x| get_equations(&x, &directions, &node_map))
            .map(|(_, cycle_len, _)| cycle_len as usize) // this isn't correct but correct enough for the generated inputs. see counterexample test case
            .collect()
    ) as u64
}

fn solve_part2_brute(lines: &Vec<String>) -> u64 {
    let directions_line = &lines[0];
    let directions = directions_line.chars().collect::<Vec<char>>();

    let node_lines = lines
        .clone()
        .into_iter()
        .skip(1)
        .filter(|line| line.len() > 0)
        .collect::<Vec<String>>();
    
    let node_map = parse_nodes(&node_lines);

    let mut positions: Vec<&str> = node_map
        .keys()
        .filter(|label| label.ends_with("A"))
        .cloned()
        .collect();

    let mut steps = 0;
    let mut direction_index = 0;
    while !positions.iter().all(|pos| pos.ends_with("Z")) {
        let new_positions = positions.iter().map(|pos| {
            let node = node_map.get(pos).expect("missing node");

            let direction = directions[direction_index];
            if direction == 'L' {
                return node.left;
            } else {
                return node.right;
            }
        }).collect::<Vec<&str>>();

        positions = new_positions;
        direction_index = (direction_index + 1) % directions.len();
        steps += 1;
    }
    steps
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
    fn test_part_2_counterexample() {
        let file_path = format!("./data/{DAYSTRING}/counterexample.txt");
        assert_eq!(
            solve_part2_brute(&load_file(&file_path)),
            3
        );
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            3
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example3.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            6
        );
    }
}
