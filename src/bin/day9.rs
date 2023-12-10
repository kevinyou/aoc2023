
use aoc2023::load_file;

static DAYSTRING: &str = "day9";

fn get_next_num(sequence: &Vec<i32>) -> i32 {
    let mut derivatives = sequence.clone();
    let mut sum = 0;

    while !derivatives.iter().all(|n| *n == 0) {
        sum += *(derivatives.iter().last().expect("length nonzero"));
        let temp = derivatives.len() - 1;
        let new_derivatives = derivatives
            .iter()
            .take(temp)
            .enumerate()
            .map(|(i, _)| derivatives[i + 1] - derivatives[i])
            .collect();
        derivatives = new_derivatives;
    }

    sum
}

fn get_prev_num(sequence: &Vec<i32>) -> i32 {
    let mut list: Vec<Vec<i32>> = Vec::new();
    list.push(sequence.clone());

    for i in 1..sequence.len() {
        let x = &list[i-1];
        let dx = x
            .clone()
            .into_iter()
            .take(x.len() - 1)
            .enumerate()
            .map(|(j, _)| x[j+1] - x[j])
            .collect::<Vec<i32>>();

        list.push(dx);
    }

    for i in (0..(list.len() - 1)).rev() {
        let x_0 = *list.get(i).unwrap().first().unwrap();
        let dx_0 = *list.get(i+1).unwrap().first().unwrap();
        let x_neg_1 = x_0 - dx_0;

        list[i].insert(0, x_neg_1);
    }

    list 
        .into_iter()
        .map(|v| v.into_iter().nth(0).or(Some(0)).unwrap())
        .nth(0)
        .unwrap()
}


fn solve_part1(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.split(' ')
        .map(|s| s.parse().expect("number in each"))
        .collect::<Vec<i32>>())
        .map(|x| get_next_num(&x))
        .sum()
}

fn solve_part2(lines: &Vec<String>) -> i32 {
    lines
        .into_iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.split(' ')
        .map(|s| s.parse().expect("number in each"))
        .collect::<Vec<i32>>())
        .map(|x| get_prev_num(&x))
        .sum()
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
            114
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            2
        );
    }
}
