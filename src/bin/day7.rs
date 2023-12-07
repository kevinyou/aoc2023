
use std::{collections::HashMap, cmp::Ordering};
use aoc2023::load_file;

static DAYSTRING: &str = "day7";

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn get_card(c: char, is_part_2: bool) -> Card {
    return match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => {
            if is_part_2 {
                return Card::Joker;
            }
            return Card::Jack;
        },
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => Card::Two,
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_type(cards: &String, is_part_2: bool) -> HandType {
    let mut map: HashMap<char, u32> = HashMap::new();
    let mut num_jokers = 0;
    for c in cards.chars() {
        if c == 'J' && is_part_2 {
            num_jokers += 1;
        } else {
            *map.entry(c).or_insert(0) += 1;
        }
    }

    let vals: Vec<u32> = map.values().cloned().collect();

    if num_jokers >= 4 {
        return HandType::FiveOfAKind
    }
    if num_jokers == 3 {
        if vals.iter().any(|x| *x == 2) {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    }

    let num_pairs = vals.iter().filter(|x| **x == 2).count();

    if vals.iter().any(|x| *x == 5) {
        return HandType::FiveOfAKind
    }
    if vals.iter().any(|x| *x == 4) {
        if num_jokers == 1 {
            return HandType::FiveOfAKind
        }
        return HandType::FourOfAKind
    }

    if vals.iter().any(|x| *x == 3) {
        if num_jokers == 2 {
            return HandType::FiveOfAKind
        }
        if num_jokers == 1 {
            return HandType::FourOfAKind
        }
        if num_pairs == 1 {
            return HandType::FullHouse
        }
        return HandType::ThreeOfAKind
    }


    if num_pairs == 2 {
        if num_jokers == 1 {
            return HandType::FullHouse
        }
        return HandType::TwoPair;
    }

    if num_pairs == 1 {
        if num_jokers == 2 {
            return HandType::FourOfAKind
        }
        if num_jokers == 1 {
            return HandType::ThreeOfAKind
        }
        return HandType::OnePair;
    }

    if num_jokers == 2 {
        return HandType::ThreeOfAKind
    }
    if num_jokers == 1 {
        return HandType::OnePair
    }
    HandType::HighCard
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

fn line_to_hands(line: &String, is_part_2: bool) -> Hand {
    let parts: Vec<&str> = line
        .split(' ')
        .collect();
    let cards = parts[0]
        .to_string();
    let hand_type = get_type(&cards, is_part_2);

    let cards = cards
        .chars()
        .map(|c| get_card(c, is_part_2))
        .collect::<Vec<Card>>();
    let cards = [
        cards[0],
        cards[1],
        cards[2],
        cards[3],
        cards[4],
    ];

    return Hand {
        cards: cards,
        bid: parts[1].trim().parse::<u32>().expect("Failed to parse bid"),
        hand_type: hand_type,
    };
}

fn get_total_winnings(hands: &mut Vec<Hand>) -> u32 {
    hands.sort_by(|a, b| {
        let hand_type_ordering = a.hand_type.cmp(&b.hand_type);
        if let Ordering::Equal = hand_type_ordering {
            return a.cards.cmp(&b.cards);
        }
        return hand_type_ordering;
    });

    // Handy for debugging!
    /*
    for hand in hands.iter() {
        println!("{:?}", &hand);
    }
    */

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| ((i + 1) as u32) * hand.bid)
        .sum()
}

fn solve_part1(lines: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = lines
        .clone()
        .iter()
        .filter(|s| s.len() > 0)
        .map(|x| line_to_hands(x, false))
        .collect();

    get_total_winnings(&mut hands)
}

fn solve_part2(lines: &Vec<String>) -> u32 {
    let mut hands: Vec<Hand> = lines
        .clone()
        .iter()
        .filter(|s| s.len() > 0)
        .map(|x| line_to_hands(x, true))
        .collect();

    get_total_winnings(&mut hands)
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
            6440
        );
    }

    #[test]
    fn test_part_2() {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            5905
        );
    }
}
