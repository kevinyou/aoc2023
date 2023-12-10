
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

    let mut elem = list[0].clone();
    while list.len() < sequence.len() {
        let new_seq = elem
            .clone()
            .into_iter()
            .take(elem.len() - 1)
            .enumerate()
            .map(|(i, _)| elem[i+1] - elem[i])
            .collect::<Vec<i32>>();
        list.push(new_seq.clone());

        elem = new_seq;
    }

    for i in (0..(list.len() - 1)).rev() {
        let future_elem = *list.get(i+1).unwrap().first().unwrap();
        let past_elem = *list.get(i).unwrap().first().unwrap();
        let elem = &mut list[i];
        elem.insert(0, past_elem - future_elem);
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
