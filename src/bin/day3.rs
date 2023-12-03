
use aoc2023::load_file;

static DAYSTRING: &str = "day3";

struct SchematicPartNumber {
    row: usize,
    col_start: usize,
    // [col_start, col_end) exclusive
    col_end: usize,
}

/**
 * Check these numbers in a grid:
 * ........
 * XXXXXXX.
 * X12345X.
 * XXXXXXX.
 */
fn check_adjacent(part: &SchematicPartNumber, grid: &Vec<Vec<char>>) -> bool {
    let char_right_of_number = grid[part.row].get(part.col_end);
    if let Some(c) = char_right_of_number {
        if is_symbol(*c) {
            return true;
        }
    }

    let mut left_index = part.col_start;
    if left_index > 0 {
        left_index = left_index - 1;

        let char_left_of_number = grid[part.row].get(left_index);
        if let Some(c) = char_left_of_number {
            if is_symbol(*c) {
                return true;
            }
        }
    }

    if part.row > 0 {
        let row_above_number = grid
            .get(part.row - 1);
        if let Some(row) = row_above_number {
            for j in left_index..=(part.col_end) {
                let maybe_c = row
                    .get(j);
                if let Some(c) = maybe_c {
                    if is_symbol(*c) {
                        return true;
                    }
                }
            }
        }
    }

    let row_below_number = grid
        .get(part.row + 1);
    if let Some(row) = row_below_number {
        for j in left_index..=(part.col_end) {
            let maybe_c = row
                .get(j);
            if let Some(c) = maybe_c {
                if is_symbol(*c) {
                    return true;
                }
            }
        }
    }

    return false;
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let mut part_number_sum = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut part_numbers: Vec<SchematicPartNumber> = Vec::new();
    for i in 0..grid.len() {
        let mut part_number_string = String::from("");
        let mut col_start = -1;
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if c.is_digit(10) {
                part_number_string.push(c);
                if col_start == -1 {
                    col_start = j as i32;
                }
            } else if part_number_string.len() > 0 {
                let elem = SchematicPartNumber {
                    row: i,
                    col_start: col_start as usize,
                    col_end: j,
                };
                part_numbers.push(elem);

                part_number_string = String::from("");
                col_start = -1;
            }
        }
        if part_number_string.len() > 0 {
            let elem = SchematicPartNumber {
                row: i,
                col_start: col_start as usize,
                col_end: (col_start as usize) + part_number_string.len(),
            };
            part_numbers.push(elem);
        }
    }

    for part_number in part_numbers {
        let part_number_value = (part_number.col_start..part_number.col_end)
            .map(|col| grid[part_number.row][col])
            .collect::<String>()
            .parse::<u32>()
            .expect("Did not form number");

        if check_adjacent(&part_number, &grid) {
            part_number_sum += part_number_value;
        }
    }

    part_number_sum
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
            4361,
        );
    }
}
