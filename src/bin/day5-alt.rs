
use aoc2023::load_file;

static DAYSTRING: &str = "day5";

#[derive(Debug, Clone)]
struct MapEntry {
    destination: i64,
    source: i64,
    len: i64,
}

impl MapEntry {
    #[allow(dead_code)]
    fn to_input_form(&self) -> String {
        return format!("{} {} {}", self.destination, self.source, self.len);
    }
}

fn parse_map_entry(line: &String) -> MapEntry {
    let line: Vec<i64> = line
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("non-number in map entry"))
        .collect();
    MapEntry {
        destination: line[0],
        source: line[1],
        len: line[2],
    }
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    // first, set up data structures
    let seeds: Vec<i64> = lines[0]
        .split(':')
        .last()
        .expect("Seed line not formatted as expected").split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("Seed not numbers"))
        .collect();

    let mut blocks = create_blocks(lines);
    blocks.reverse();
    // now work backwards
    let mut loc = 0;
    loop {
        let mut val = loc;
        for block in blocks.iter() {
            for entry in block {
                // Use map backwards
                if entry.destination <= val && val < entry.destination + entry.len {
                    val = entry.source + val - entry.destination;
                    break;
                }
            }
        }
        if seeds.iter().any(|seed| *seed == val) {
            return loc;
        }
        loc += 1;
    }
}

fn create_blocks(lines: &Vec<String>) -> Vec<Vec<MapEntry>> {
    let mut blocks: Vec<Vec<MapEntry>> = Vec::new();
    let mut block = Vec::new();
    for line in lines.into_iter().skip(2) {
        if line == "" {
            if block.len() > 0 {
                blocks.push(block);
                block = Vec::new();
            }
            continue;
        }

        if line.chars().nth(0).unwrap().is_alphabetic() {
            continue;
        }

        let new_function = parse_map_entry(line);
        block.push(new_function);
    }

    return blocks;
}

/**
 * Does the domain of a new function overlap with the range of any existing functions?
 * For each overlapping pair, update the existing functions as needed, and then after making changes to the new function (and its domain/range/len), add the new function..
 * Sort all functions in ascending order by their ranges.
 * In ascending order of function ranges, check if there's an overlap between the seeds (domain!) and function's range. (Sound familiar?)
 * If there is an overlap, return the start value of the overlap.
 */
fn solve_part2(lines: &Vec<String>) -> i64 {
    // first, set up data structures
    let seeds: Vec<i64> = lines[0]
        .split(':')
        .last()
        .expect("Seed line not formatted as expected").split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("Seed not numbers"))
        .collect();
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        pairs.push((seeds[i], seeds[i as usize + 1]));
    }

    let mut blocks = create_blocks(lines);
    blocks.reverse();
    // now work backwards
    let mut loc = 0;
    loop {
        let mut val = loc;
        // println!("{}", val);
        for block in blocks.iter() {
            for entry in block {
                // Use map backwards
                if entry.destination <= val && val < entry.destination + entry.len {
                    val = entry.source + val - entry.destination;
                    break;
                }
            }
        }
        if pairs.iter().any(|(start, len)| *start <= val && val < *start + *len) {
            return loc;
        }
        loc += 1;
    }
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
            35
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            46
        );
    }
}
