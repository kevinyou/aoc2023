
use aoc2023::load_file;
use std::str::FromStr;

static DAYSTRING: &str = "day2";


enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {

    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

struct Clue {
    count: u32,
    color: Color,
}


fn parse_clue(clue: &str) -> Clue{
    let parts: Vec<&str> = clue.trim().split(' ').collect();
    Clue {
        count: parts[0].parse().expect("Clue count not number"),
        color: Color::from_str(parts[1]).expect("Clue color not color"),
    }
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let mut game_id_sum = 0;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let v: Vec<&str> = line.split(':').collect();

        let game_id = v[0];
        let game_id: Vec<&str> = game_id.split(' ').collect();
        let game_id: u32 = game_id[1]
            .parse()
            .expect("Game id is not a number");

        let sets = v[1].trim();
        let sets: Vec<&str> = sets.split(';').collect();
        let sets: Vec<Vec<Clue>> = sets
            .into_iter()
            .map(
                |set| set.split(", ")
                .map(parse_clue)
                .collect()
            )
            .collect();

        let mut valid = true;
        for set in sets {
            for clue in set {
                let quantity = match clue.color {
                    Color::Red => 12,
                    Color::Blue => 13,
                    Color::Green => 14,
                };
                if clue.count > quantity {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            game_id_sum += game_id;
        }
    }
    game_id_sum
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
