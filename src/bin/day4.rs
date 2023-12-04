
use std::collections::HashSet;
use std::collections::HashMap;
use aoc2023::load_file;

static DAYSTRING: &str = "day4";

fn solve_part1(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let line: Vec<&str> = line.split(':').collect();
            let sides: Vec<&str> = line[1].trim().split('|').collect();

            let winning_numbers = sides[0]
                .trim()
                .split(' ')
                .filter(|s| s.len() > 0);
            let winning_numbers: HashSet<&str> = HashSet::from_iter(winning_numbers);

            let card_numbers = sides[1]
                .trim()
                .split(' ')
                .filter(|s| s.len() > 0);

            let num_winning = card_numbers
                .filter(|n| winning_numbers.contains(n))
                .count() as u32;
            if num_winning == 0 {
                return 0;
            }
            u32::pow(2, num_winning - 1)
        })
        .sum()
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let mut card_num_winning: HashMap<String, u32> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();

    lines
        .into_iter()
        .filter(|line| line.len() > 0)
        .for_each(|line| {
            let line: Vec<&str> = line.split(':').collect();

            let card_id = line[0].split(' ').last().expect("Card id missing").to_string();

            let sides: Vec<&str> = line[1].trim().split('|').collect();

            let winning_numbers = sides[0]
                .trim()
                .split(' ')
                .filter(|s| s.len() > 0);
            let winning_numbers: HashSet<&str> = HashSet::from_iter(winning_numbers);

            let card_numbers = sides[1]
                .trim()
                .split(' ')
                .filter(|s| s.len() > 0);

            let num_winning = card_numbers
                .filter(|n| winning_numbers.contains(n))
                .count() as u32;
            card_num_winning.insert(card_id.clone(), num_winning);
            stack.push(card_id.clone());
        });

    let mut total_cards = 0;
    while stack.len() > 0 {
        let card_id = stack.pop().expect("stack empty");
        total_cards += 1;

        let num_won = card_num_winning.get(&card_id).expect("id missing from map");

        let card_id: u32 = card_id.parse().expect("parse failed");
        for i in (card_id + 1)..=(card_id + num_won) {
            stack.push(i.to_string());
        }
    }

    total_cards
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
            13,
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            30,
        );
    }
}
