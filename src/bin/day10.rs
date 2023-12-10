    
use std::collections::HashSet;

use aoc2023::load_file;

static DAYSTRING: &str = "day10";

// for debug purposes
#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn expand_c(c: char) -> [[char; 3]; 3] {
    match c {
        '.' => [
            ['.', '.', '.'],
            ['.', '.', '.'],
            ['.', '.', '.'],
        ],
        '|' => [
            ['.', 'X', '.'],
            ['.', 'X', '.'],
            ['.', 'X', '.'],
        ],
        '-' => [
            ['.', '.', '.'],
            ['X', 'X', 'X'],
            ['.', '.', '.'],
        ],
        'L' => [
            ['.', 'X', '.'],
            ['.', 'X', 'X'],
            ['.', '.', '.'],
        ],
        'F' => [
            ['.', '.', '.'],
            ['.', 'X', 'X'],
            ['.', 'X', '.'],
        ],
        'J' => [
            ['.', 'X', '.'],
            ['X', 'X', '.'],
            ['.', '.', '.'],
        ],
        '7' => [
            ['.', '.', '.'],
            ['X', 'X', '.'],
            ['.', 'X', '.'],
        ],
        'S' => [
            ['.', '?', '.'],
            ['?', '?', '?'],
            ['.', '?', '.'],
        ],
        _ => panic!("unexpected c {c}")
    }
}

fn expand(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded_grid: Vec<Vec<char>> = Vec::new();
    for row in grid.iter() {
        let mut expanded_row0 = Vec::new();
        let mut expanded_row1 = Vec::new();
        let mut expanded_row2 = Vec::new();
        for c in row {
            let expanded_c = expand_c(*c);
            expanded_row0.extend_from_slice(&expanded_c[0]);
            expanded_row1.extend_from_slice(&expanded_c[1]);
            expanded_row2.extend_from_slice(&expanded_c[2]);
        }
        expanded_grid.push(expanded_row0);
        expanded_grid.push(expanded_row1);
        expanded_grid.push(expanded_row2);
    }

    expanded_grid
}

fn flood_fill(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stack: Vec<Point> = vec![Point {
        x: 0,
        y: 0,
    }];

    let ds = [
        Point {
            x: 1,
            y: 0
        },
        Point {
            x: -1,
            y: 0
        },
        Point {
            x: 0,
            y: 1
        },
        Point {
            x: 0,
            y: -1
        },
    ];

    while !stack.is_empty() {
        let point = stack.pop().unwrap();

        if grid[point.x as usize][point.y as usize] == '.' {
            grid[point.x as usize][point.y as usize] = 'W';

            for d in ds {
                let potential_coord = Point {
                    x: point.x + d.x,
                    y: point.y + d.y,
                };
                if potential_coord.is_valid(grid.len(), grid[0].len()) {
                    stack.push(potential_coord);
                }
            }
        }

    }

    grid
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct StartingCheck {
    d: Point,
    valid_cs: [char; 3],
    traveling_dir: Direction,
}

impl Point {
    pub fn is_valid(self, max_x: usize, max_y: usize) -> bool{
        let max_x: i32 = max_x as i32;
        let max_y: i32 = max_y as i32;

        return 0 <= self.x && self.x < max_x && 0 <= self.y && self.y < max_y;
    }
}

fn apply_next_dir(next_dir: &Direction, point: &Point) -> Point {
    match next_dir {
        Direction::Up => Point {
            x: point.x - 1,
            y: point.y
        },
        Direction::Down => Point {
            x: point.x + 1,
            y: point.y
        },
        Direction::Left => Point {
            x: point.x,
            y: point.y - 1,
        },
        Direction::Right => Point {
            x: point.x,
            y: point.y + 1,
        },
    }
}

fn determine_next_dir(from_dir: &Direction, c: char) -> Direction {
    match from_dir {
        Direction::Up => match c {
            '|' => Direction::Up,
            'F' => Direction::Right,
            '7' => Direction::Left,
            _ => panic!("unexpected from dir {:?} for c {}", from_dir, c)
        },
        Direction::Down => match c {
            '|' => Direction::Down,
            'L' => Direction::Right,
            'J' => Direction::Left,
            _ => panic!("unexpected from dir {:?} for c {}", from_dir, c)
        },
        Direction::Left => match c {
            '-' => Direction::Left,
            'L' => Direction::Up,
            'F' => Direction::Down,
            _ => panic!("unexpected from dir {:?} for c {}", from_dir, c)
        },
        Direction::Right => match c {
            '-' => Direction::Right,
            'J' => Direction::Up,
            '7' => Direction::Down,
            _ => panic!("unexpected from dir {:?} for c {}", from_dir, c)
        },
    }
}

fn get_clean_grid(grid: &Vec<Vec<char>>, start: &Point) -> Vec<Vec<char>> {
    let starting_checks = [
        StartingCheck {
            d: Point {
                x: 1,
                y: 0
            },
            valid_cs: ['|', 'L', 'J'],
            traveling_dir: Direction::Down,
        },
        StartingCheck {
            d: Point {
                x: -1,
                y: 0
            },
            valid_cs: ['|', 'F', '7'],
            traveling_dir: Direction::Up,
        },
        StartingCheck {
            d: Point {
                x: 0,
                y: 1
            },
            valid_cs: ['-', 'J', '7'],
            traveling_dir: Direction::Right,
        },
        StartingCheck {
            d: Point {
                x: 0,
                y: -1
            },
            valid_cs: ['-', 'L', 'F'],
            traveling_dir: Direction::Left,
        },
    ];

    let mut point = start.clone();
    let mut traveling_dir: Direction = Direction::Down;
    let mut first_dir: Direction = Direction::Down;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut clean_grid = grid.clone();

    visited.insert(point);

    // Find the first char adjacent to S that connects correctly
    // i.e. 
    for starting_check in starting_checks {
        let d = starting_check.d;
        let valid_cs = starting_check.valid_cs;
        let potential_coord = Point {
            x: point.x + d.x,
            y: point.y + d.y,
        };

        if potential_coord.is_valid(grid.len(), grid[0].len()) {
            let c = grid[potential_coord.x as usize][potential_coord.y as usize];
            if valid_cs.contains(&c) {
                point = potential_coord;
                visited.insert(point);
                first_dir = starting_check.traveling_dir.clone();
                let original_traveling_dir = starting_check.traveling_dir;
                traveling_dir = determine_next_dir(&original_traveling_dir, c);
                break;
            }
        }
    }

    let mut last_dir: Direction = Direction::Down;
    while point != *start {
        let new_point = apply_next_dir(&traveling_dir, &point);
        let c = grid[new_point.x as usize][new_point.y as usize];
        if c == 'S' {
            last_dir = traveling_dir;
            break;
        }
        let new_traveling_dir = determine_next_dir(&traveling_dir, c);
        // apply next_dir to point

        point = new_point;
        traveling_dir = new_traveling_dir;
        visited.insert(point);
    }

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let checking_point = Point {
                x: x as i32,
                y: y as i32,
            };
            if !visited.contains(&checking_point) {
                clean_grid[x][y] = '.';
            } else if grid[checking_point.x as usize][checking_point.y as usize] == 'S' {
                clean_grid[x][y] = match first_dir {
                    Direction::Down => match last_dir {
                        Direction::Left => 'F',
                        Direction::Right=> '7',
                        Direction::Down=> '|',
                        _ => panic!("did not expect first_dir {:?} last_dir {:?}", first_dir, last_dir),
                    },
                    Direction::Up => match last_dir {
                        Direction::Left => 'L',
                        Direction::Right => 'J',
                        Direction::Up => '|',
                        _ => panic!("did not expect first_dir {:?} last_dir {:?}", first_dir, last_dir),
                    },
                    Direction::Left => match last_dir {
                        Direction::Up => '7',
                        Direction::Down => 'J',
                        Direction::Left => '-',
                        _ => panic!("did not expect first_dir {:?} last_dir {:?}", first_dir, last_dir),
                    },
                    Direction::Right => match last_dir {
                        Direction::Up => 'F',
                        Direction::Down => 'L',
                        Direction::Right => '-',
                        _ => panic!("did not expect first_dir {:?} last_dir {:?}", first_dir, last_dir),
                    },
                };
            }
        }
    }


    clean_grid
}

fn get_loop_length(grid: &Vec<Vec<char>>, start: &Point) -> usize {
    let starting_checks = [
        StartingCheck {
            d: Point {
                x: 1,
                y: 0
            },
            valid_cs: ['|', 'L', 'J'],
            traveling_dir: Direction::Down,
        },
        StartingCheck {
            d: Point {
                x: -1,
                y: 0
            },
            valid_cs: ['|', 'F', '7'],
            traveling_dir: Direction::Up,
        },
        StartingCheck {
            d: Point {
                x: 0,
                y: 1
            },
            valid_cs: ['-', 'J', '7'],
            traveling_dir: Direction::Right,
        },
        StartingCheck {
            d: Point {
                x: 0,
                y: -1
            },
            valid_cs: ['-', 'L', 'F'],
            traveling_dir: Direction::Left,
        },
    ];

    let mut loop_length = 0;
    let mut point = start.clone();
    let mut traveling_dir: Direction = Direction::Down;

    // Find the first char adjacent to S that connects correctly
    // i.e. 
    for starting_check in starting_checks {
        let d = starting_check.d;
        let valid_cs = starting_check.valid_cs;
        let potential_coord = Point {
            x: point.x + d.x,
            y: point.y + d.y,
        };

        if potential_coord.is_valid(grid.len(), grid[0].len()) {
            let c = grid[potential_coord.x as usize][potential_coord.y as usize];
            if valid_cs.contains(&c) {
                loop_length = 1;
                point = potential_coord;
                let original_traveling_dir = starting_check.traveling_dir;
                traveling_dir = determine_next_dir(&original_traveling_dir, c);
                break;
            }
        }
    }

    while point != *start {
        let new_point = apply_next_dir(&traveling_dir, &point);
        let c = grid[new_point.x as usize][new_point.y as usize];
        if c == 'S' {
            loop_length += 1;
            break;
        }
        let new_traveling_dir = determine_next_dir(&traveling_dir, c);
        // apply next_dir to point

        point = new_point;
        traveling_dir = new_traveling_dir;
        loop_length += 1;
    }

    // Then follow the path 

    loop_length
}

fn solve_part1(lines: &Vec<String>) -> usize {
    let grid: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .map(|elem| elem.chars().collect())
        .collect();

    let mut point = Point {
        x: 0,
        y: 0,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                point.x = i as i32;
                point.y = j as i32;
                break;
            }
        }
    }

    get_loop_length(&grid, &point) / 2
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let grid: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .map(|elem| elem.chars().collect())
        .collect();

    let mut point = Point {
        x: 0,
        y: 0,
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                point.x = i as i32;
                point.y = j as i32;
                break;
            }
        }
    }

    // first, traverse the grid to mark all parts of the path, and namely, unmark all parts not in the path
    // this includes changing S to the actual shape.
    let clean_grid = get_clean_grid(&grid, &point);
    // then, expand the grid
    let expanded_grid = expand(&clean_grid);

    // run flood fill from the top left corner (guaranteed to be outside)
    let expanded_grid = flood_fill(expanded_grid);

    let mut count = 0;
    // then count how many 3x3 grids are still dots.
    for x in (0..expanded_grid.len()).step_by(3) {
        for y in (0..expanded_grid[0].len()).step_by(3) {
            let mut is_unfilled = true;
            for i in 0..2 {
                for j in 0..2 {
                    if expanded_grid[x+j][y+i] != '.' {
                        is_unfilled = false;
                        break;
                    }
                }
            }

            if is_unfilled {
                count += 1;
            }
        }
    }
    count
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
    fn test_expand() {
        let example_3 = format!("./data/{DAYSTRING}/example6.txt");

        let lines = &load_file(&example_3);
        let grid: Vec<Vec<char>> = lines
            .clone()
            .into_iter()
            .map(|elem| elem.chars().collect())
            .collect();
        
        print_grid(&grid);

        let expanded_grid = expand(&grid);

        print_grid(&expanded_grid);

        // panic!("boom");
    }

    #[test]
    fn test_part_2() {
        let example_3 = format!("./data/{DAYSTRING}/example3.txt");
        assert_eq!(
            solve_part2(&load_file(&example_3)),
            4
        );

        let example_4 = format!("./data/{DAYSTRING}/example4.txt");
        assert_eq!(
            solve_part2(&load_file(&example_4)),
            4
        );

        let example_5 = format!("./data/{DAYSTRING}/example5.txt");
        assert_eq!(
            solve_part2(&load_file(&example_5)),
            8
        );

        let example_6 = format!("./data/{DAYSTRING}/example6.txt");
        assert_eq!(
            solve_part2(&load_file(&example_6)),
            10
        );
    }
}
