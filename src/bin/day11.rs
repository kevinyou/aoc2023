
use aoc2023::load_file;

static DAYSTRING: &str = "day11";

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn cosmic_expansion(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_rows: Vec<usize> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| {
            row.iter().all(|c| *c == '.')
        })
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<usize> = grid[0]
        .iter()
        .enumerate()
        .filter(|(j, _)| {
            let cs: Vec<char> = grid
                .iter()
                .map(|row| row[*j])
                .collect();
            cs.iter().all(|c| *c == '.')
        })
        .map(|(j, _)| j)
        .collect();

    let mut new_grid = grid;

    // add empty cols
    for j in empty_cols.into_iter().rev().collect::<Vec<usize>>() {
        for row in new_grid.iter_mut() {
            row.insert(j, '.');
        }
    }

    // add empty rows
    for i in empty_rows.into_iter().rev().collect::<Vec<usize>>() {
        new_grid.insert(i, new_grid[i].clone());
    }

    new_grid
}

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .clone()
        .into_iter()
        .map(|elem| elem.chars().collect())
        .collect()
}

#[allow(unused_variables)]
fn solve_part1(lines: &Vec<String>) -> u32 {
    let grid = parse_grid(lines);
    print_grid(&grid);
    let grid = cosmic_expansion(grid);
    print_grid(&grid);
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
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part1(&load_file(&file_path)),
            374
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
