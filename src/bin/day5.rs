
use std::cmp::Ordering;

use aoc2023::load_file;

static DAYSTRING: &str = "day5";

enum OverlapType {
    None,
    PartialNewHigher,
    PartialOldHigher,
    NewInOld,
    OldInNew,
}

#[derive(Debug, Clone)]
struct Interval {
    // Inclusive
    start: i64,
    // Inclusive
    end: i64,
}

#[derive(Debug, Clone)]
struct MapFunction {
    domain: Interval,
    range: Interval,
}

impl Ord for MapFunction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.range.start.cmp(&other.range.start)
    }
}

impl PartialOrd for MapFunction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapFunction {
    fn eq(&self, other: &Self) -> bool {
        self.range.start == other.range.start
    }
}

impl Eq for MapFunction {}


fn get_overlap_type(old: &MapFunction, new: &MapFunction) -> OverlapType {
    if new.domain.start >= old.range.start && new.domain.start < old.range.end {
        if new.domain.end <= old.range.end {
            return OverlapType::NewInOld;
        } else  {
            return OverlapType::PartialNewHigher;
        }
    }

    if old.domain.start >= new.range.start && old.domain.start < new.range.end {
        if old.domain.end <= new.range.end {
            return OverlapType::OldInNew;
        } else  {
            return OverlapType::PartialOldHigher;
        }
    }

    return OverlapType::None;
}

#[derive(Debug)]
struct MapEntry {
    source: i64,
    destination: i64,
    len: i64,
}

type Map = Vec<MapEntry>;

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

fn parse_map_function(line: &String) -> MapFunction {
    let line: Vec<i64> = line
        .split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("non-number in map entry"))
        .collect();
    MapFunction {
        domain: Interval {
            start: line[1],
            end: line[1] + line[2] - 1,
        },
        range: Interval {
            start: line[0],
            end: line[0] + line[2] - 1,
        },
    }
}

fn handle_full_subset(outer_function: &MapFunction, inner_function: &MapFunction) -> Vec<MapFunction> {
    let left_old_domain_intersection_point = (outer_function.domain.start  - outer_function.range.start) + inner_function.domain.start;
    let right_old_domain_intersection_point = (outer_function.domain.start  - outer_function.range.start) + inner_function.domain.end;

        let left = MapFunction {
        domain: Interval {
            start: outer_function.domain.start,
            end: left_old_domain_intersection_point - 1,
        },
        range: Interval {
            start: outer_function.range.start,
            end: inner_function.domain.start - 1,
        },
        };

        let middle = MapFunction {
        domain: Interval {
            start: left_old_domain_intersection_point,
            end: right_old_domain_intersection_point,
        },
        range: Interval {
            start: inner_function.range.start,
            end: inner_function.range.end,
        },
        };

        let right = MapFunction {
        domain: Interval {
            start: right_old_domain_intersection_point + 1,
            end: outer_function.domain.end,
        },
        range: Interval {
            start: inner_function.domain.end + 1,
            end: outer_function.range.end,
        },
        };

        let mut pieces = Vec::new();

        if left.domain.end > left.domain.start && left.domain.end >= 0 && left.range.end > 0 {
            pieces.push(left);
        }
        pieces.push(middle);
        if right.domain.end > right.domain.start {
            pieces.push(right);
        }

        return pieces;
}


fn handle_partial_overlap(higher_function: &MapFunction, lower_function: &MapFunction) -> Vec<MapFunction> {
    let lower_range_left_intersction_point = higher_function.domain.start;
    let lower_domain_left_intersction_point = higher_function.domain.start + (lower_function.domain.start - lower_function.range.start);
    let left = MapFunction {
        domain: Interval {
            start: lower_function.domain.start,
            end: lower_domain_left_intersction_point - 1,
        },
        range: Interval {
            start: lower_function.range.start,
            end: lower_range_left_intersction_point - 1,
        },
    };

    let higher_domain_right_intersection_point = lower_function.range.end;
    let higher_range_right_intersection_point = lower_function.range.end + (higher_function.range.start - higher_function.domain.start);

    let middle = MapFunction {
        domain: Interval {
            start: lower_domain_left_intersction_point,
            end: lower_function.domain.end,
        },
        range: Interval {
            start: higher_function.range.start,
            end: higher_range_right_intersection_point,
        },
    };

    let right = MapFunction {
        domain: Interval {
            start: higher_domain_right_intersection_point + 1,
            end: higher_function.domain.end,
        },
        range: Interval {
            start: higher_range_right_intersection_point + 1,
            end: higher_function.range.end,
        },
    };

    let mut pieces = Vec::new();
    if left.domain.end > left.domain.start && left.domain.end >= 0 && left.range.end > 0 {
        pieces.push(left);
    }
    pieces.push(middle);
    if right.domain.end > right.domain.start {
        pieces.push(right);
    }
    return pieces;
}


fn layer(source: i64, map: &Map) -> i64 {
    let entry: Vec<&MapEntry> = map
        .into_iter()
        .filter(|entry| source >= entry.source && source < entry.source + entry.len)
        .collect();
    if entry.len() > 0 {
        // only one matching range
        // assert_eq!(entry.len(), 1);

        let entry = entry[0];
        let delta = source - entry.source;
        return entry.destination + delta;
    }
    return source;
}

fn layer_part2(source: i64, map: &Vec<MapFunction>) -> i64 {
    let entry: Vec<&MapFunction> = map
        .into_iter()
        .filter(|f| source >= f.domain.start && source <= f.domain.end)
        .collect();
    if entry.len() > 0 {
        // only one matching range
        /*
        if entry.len() != 1 {
            println!("{:?}", source);
            println!("{:#?}", entry);
            assert_eq!(1, 0, "panik");
        }
        assert_eq!(entry.len(), 1);
        */

        let entry = entry[0];
        let delta = source - entry.domain.start;
        let output = entry.range.start + delta;
        // println!("{:?}", source);
        // println!("{:#?}", entry);
        // println!("{:?}", output);
        return output;
    }
    return source;
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let seeds: Vec<i64> = lines[0]
        .split(':')
        .last()
        .expect("Seed line not formatted as expected").split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("Seed not numbers"))
        .collect();

    // 2 skips ?
    let mut i = 0;
    i += 2; // skip seed lines

    let mut seed_to_soil = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        seed_to_soil.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut soil_to_fertilizer = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        soil_to_fertilizer.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut fertilizer_to_water = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        fertilizer_to_water.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut water_to_light = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        water_to_light.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut light_to_temperature = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        light_to_temperature.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut temperature_to_humidity = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        temperature_to_humidity.push(parse_map_entry(&lines[i]));
        i += 1;
    }
    i += 1; // skip trailing

    let mut humidity_to_location = Map::new();
    i += 1; // skip title
    while i < lines.len() && lines[i].len() > 0 {
        humidity_to_location.push(parse_map_entry(&lines[i]));
        i += 1;
    }

    let layer1: Vec<i64> = seeds
        .clone()
        .into_iter()
        .map(|source| layer(source, &seed_to_soil))
        .collect();

    seeds
        .into_iter()
        .map(|source| layer(source, &seed_to_soil))
        .map(|source| layer(source, &soil_to_fertilizer))
        .map(|source| layer(source, &fertilizer_to_water))
        .map(|source| layer(source, &water_to_light))
        .map(|source| layer(source, &light_to_temperature))
        .map(|source| layer(source, &temperature_to_humidity))
        .map(|source| layer(source, &humidity_to_location))
        .min()
        .expect("does not have min")
}

/**
 * Does the domain of a new function overlap with the range of any existing functions?
 * For each overlapping pair, update the existing functions as needed, and then after making changes to the new function (and its domain/range/len), add the new function..
 * Sort all functions in ascending order by their ranges.
 * In ascending order of function ranges, check if there's an overlap between the seeds (domain!) and function's range. (Sound familiar?)
 * If there is an overlap, return the start value of the overlap.
 */
fn solve_part2(lines: &Vec<String>) -> i64 {
    let seeds: Vec<i64> = lines[0]
        .split(':')
        .last()
        .expect("Seed line not formatted as expected").split(' ')
        .filter(|s| s.len() > 0)
        .map(|s| s.parse().expect("Seed not numbers"))
        .collect();


    let mut blocks: Vec<Vec<MapFunction>> = Vec::new();
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

        let new_function = parse_map_function(line);
        block.push(new_function);
    }

    let mut fs: Vec<MapFunction> = Vec::new();

    let mut block_pending: Vec<MapFunction> = Vec::new();
    for block in blocks {
        let mut stack: Vec<MapFunction> = block.clone();
        stack.reverse(); // not needed, really

        while stack.len() > 0 {
            let new_function = stack.pop().expect("Empty stack");

            let mut index_to_remove: Option<usize> = None;
            // let mut old_function: Option<MapFunction> = None;
            for (i, old_function ) in fs.iter().enumerate() {
                let ot = get_overlap_type(&old_function, &new_function);

                match ot {
                    OverlapType::None => {
                        continue;
                    },
                    OverlapType::NewInOld => {
                        index_to_remove = Some(i);
                        let mut x = handle_full_subset(&old_function, &new_function);
                        // because of sorting,
                        // left and middle guaranteed not to have any more conflicts - they can go directly
                        // into block_pending.
                        // only right needs to go on the stack
                        stack.append(&mut x);
                        break;
                    },
                    OverlapType::OldInNew => {
                        index_to_remove = Some(i);
                        let mut x = handle_full_subset(&new_function, &old_function);
                        stack.append(&mut x);
                        break;
                    },
                    OverlapType::PartialNewHigher => {
                        index_to_remove = Some(i);
                        let mut x = handle_partial_overlap(&new_function, &old_function);
                        stack.append(&mut x);
                        break;
                    },
                    OverlapType::PartialOldHigher => {
                        index_to_remove = Some(i);
                        let mut x = handle_partial_overlap(&old_function, &new_function);
                        stack.append(&mut x);
                        break;
                    },
                };
            }

            if let Some(i) = index_to_remove {
                fs.remove(i);
            } else {
                block_pending.push(new_function);
            }

            fs.sort();
        }

        fs.append(&mut block_pending);
        block_pending = Vec::new();
        // println!("{:#?}", fs);
    }
    if block_pending.len() > 0 {
        fs.append(&mut block_pending);
    }

    seeds
        .into_iter()
        .map(|source| layer_part2(source, &fs))
        .min()
        .expect("does not have min")
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
            35
        );
    }

    #[test]
    fn test_handle_full_subset() {
        let a = MapFunction {
            domain: Interval {
                start: 1001,
                end: 1010,
            },
            range: Interval {
                start: 1,
                end: 10,
            },
        };

        let b = MapFunction {
            domain: Interval {
                start: 3,
                end: 6,
            },
            range: Interval {
                start: 103,
                end: 106,
            },
        };

        let result = handle_full_subset(&a, &b);
        let left = &result[0];
        assert_eq!(left.domain.start, 1001);
        assert_eq!(left.domain.end, 1002);
        assert_eq!(left.range.start, 1);
        assert_eq!(left.range.end, 2);

        let middle = &result[1];
        assert_eq!(middle.domain.start, 1003);
        assert_eq!(middle.domain.end, 1006);
        assert_eq!(middle.range.start, 103);
        assert_eq!(middle.range.end, 106);

        let right = &result[2];
        assert_eq!(right.domain.start, 1007);
        assert_eq!(right.domain.end, 1010);
        assert_eq!(right.range.start, 7);
        assert_eq!(right.range.end, 10);

    }

    #[test]
    fn test_handle_partial_overlap() {
        let a = MapFunction {
            domain: Interval {
                start: 1001,
                end: 1010,
            },
            range: Interval {
                start: 1,
                end: 10,
            },
        };

        let b = MapFunction {
            domain: Interval {
                start: 3,
                end: 15,
            },
            range: Interval {
                start: 103,
                end: 115,
            },
        };

        let result = handle_partial_overlap(&b, &a);
        let left = &result[0];
        assert_eq!(left.domain.start, 1001, "left");
        assert_eq!(left.domain.end, 1002, "left");
        assert_eq!(left.range.start, 1, "left");
        assert_eq!(left.range.end, 2, "left");

        let middle = &result[1];
        assert_eq!(middle.domain.start, 1003, "middle");
        assert_eq!(middle.domain.end, 1010);
        assert_eq!(middle.range.start, 103);
        assert_eq!(middle.range.end, 110);

        let right = &result[2];
        assert_eq!(right.domain.start, 11, "right");
        assert_eq!(right.domain.end, 15);
        assert_eq!(right.range.start, 111);
        assert_eq!(right.range.end, 115);

    }

    // #[test]
    fn test_55() {
        let file_path = format!("./data/{DAYSTRING}/extra1.txt");
        assert_eq!(
            solve_part2(&load_file(&file_path)),
            35
        );
    }
}
