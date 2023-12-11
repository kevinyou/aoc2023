
use aoc2023::load_file;

static DAYSTRING: &str = "day11";

// TODO: move these into common utilities
#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn dist(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

}

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
fn solve_part1(lines: &Vec<String>) -> i64 {
    let grid = parse_grid(lines);
    let grid = cosmic_expansion(grid);

    let mut galaxies: Vec<Point> = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                galaxies.push(Point {
                    x: i as i64,
                    y: j as i64,
                })
            }
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            sum += galaxies[i].dist(galaxies[j]);
        }
    }
    sum
}

fn solve_part2(lines: &Vec<String>, factor: i64) -> i64 {
    let grid = parse_grid(lines);

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

    let mut galaxies: Vec<Point> = Vec::new();
    let mut expanded_x = 0;
    for arr_x in 0..grid.len() {
        let mut expanded_y = 0;
        for arr_y in 0..grid[0].len() {
            if grid[arr_x][arr_y] == '#' {
                galaxies.push(Point {
                    x: expanded_x,
                    y: expanded_y,
                });
            }
            if empty_cols.contains(&arr_y) {
                expanded_y += factor;
            }
            expanded_y += 1;
        }
        if empty_rows.contains(&arr_x) {
            expanded_x += factor;
        }
        expanded_x += 1;
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            sum += galaxies[i].dist(galaxies[j]);
        }
    }
    sum
}


fn main() {
    // let lines = load_from_stdin();
    let file_path = format!("./data/{DAYSTRING}/part1.txt");
    let lines = load_file(&file_path);

    let part1 = solve_part1(&lines);

    println!("Part 1: {part1}");

    let part2 = solve_part2(&lines, 1_000_000 - 1);

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
    fn test_part_2_2x() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path), 2-1),
            374
        );
    }

    #[test]
    fn test_part_2_10x() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path), 10-1),
            1030
        );
    }

    #[test]
    fn test_part_2_100x() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path), 100-1),
            8410
        );
    }
}
