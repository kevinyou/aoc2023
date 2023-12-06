
use aoc2023::load_file;

static DAYSTRING: &str = "day5";

#[derive(Debug, Clone)]
struct MapEntry {
    destination: i64,
    source: i64,
    len: i64,
}

impl MapEntry {
    #[allow(dead_code)]
    fn to_input_form(&self) -> String {
        return format!("{} {} {}", self.destination, self.source, self.len);
    }
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

// one layer of maps
fn collapse(existing: &mut Vec<MapEntry>, new_block: &mut Vec<MapEntry>) {
    // Precondition: existing is sorted by dest

    // Sort new_block by source
    new_block
        .sort_by(|a, b| a.source.cmp(&b.source));

    let mut add_to_existing: Vec<MapEntry> = Vec::new();

    for new_entry in new_block {
        let mut has_overlap = false;
        for old_entry in existing.iter_mut() {
            if new_entry.source <= old_entry.destination && old_entry.destination < new_entry.source + new_entry.len {
                if new_entry.source + new_entry.len > old_entry.destination + old_entry.len {
                    // need three parts T_T but at least none are mutates
                    let overlap_size = old_entry.len;

                    let middle_new = MapEntry {
                        destination: new_entry.destination - new_entry.source + old_entry.destination,
                        source: old_entry.destination,
                        len: overlap_size,
                    };
                    let left_new = MapEntry {
                        destination: new_entry.destination,
                        source: new_entry.source,
                        len: middle_new.source - new_entry.source,
                    };
                    let right_new = MapEntry {
                        destination: middle_new.destination + middle_new.len,
                        source: middle_new.source + middle_new.len,
                        len: new_entry.len - overlap_size - (left_new.len),
                    };

                    // merge middle_new and old_entry
                    let merged = MapEntry {
                        destination: middle_new.destination,
                        source: old_entry.source,
                        len: overlap_size,
                    };
                    // assign merged to old_entry
                    old_entry.destination = merged.destination;
                    old_entry.source = merged.source;
                    old_entry.len = merged.len;
                    // push left_new and right_new to add to existing
                    add_to_existing.push(left_new);
                    stack.push(right_new);
                    // add_to_existing.push(right_new);
                    continue;
                }
                // split new_entry into two parts to exactly fit!
                let overlap_size = std::cmp::min(old_entry.len, new_entry.source + new_entry.len - old_entry.destination);
                let left_new = MapEntry {
                    destination: new_entry.destination,
                    source: new_entry.source,
                    len: new_entry.len - overlap_size,
                };
                let right_new = MapEntry {
                    destination: new_entry.destination - new_entry.source + old_entry.destination,
                    source: old_entry.destination,
                    len: overlap_size,
                };

                if right_new.len == old_entry.len {
                    // merge right_new with old_entry
                    let merged = MapEntry {
                        destination: right_new.destination,
                        source: old_entry.source,
                        len: overlap_size,
                    };
                    // assign merged to old_entry
                    old_entry.destination = merged.destination;
                    old_entry.source = merged.source;
                    old_entry.len = merged.len;
                    if left_new.len > 0 {
                        add_to_existing.push(left_new);
                    }
                } else {
                    let left_old = MapEntry {
                        destination: old_entry.destination,
                        source: old_entry.source,
                        len: overlap_size
                    };
                    let right_old = MapEntry {
                        destination: old_entry.destination + overlap_size,
                        source: old_entry.source + overlap_size,
                        len: old_entry.len - overlap_size,
                    };
                    // merge right_new and left_old
                    let merged = MapEntry {
                        destination: right_new.destination,
                        source: left_old.source,
                        len: overlap_size,
                    };
                    // add left_new and merged to array
                    if left_new.len > 0 {
                        add_to_existing.push(left_new);
                    }
                    add_to_existing.push(merged);
                    // update old_entry to right_old
                    old_entry.destination = right_old.destination;
                    old_entry.source = right_old.source;
                    old_entry.len = right_old.len;
                }
            } else if old_entry.destination <= new_entry.source && new_entry.source < old_entry.destination + old_entry.len {
                // omg. at least no contains to deal with :'()

                let overlap_size = std::cmp::min(new_entry.len, old_entry.destination + old_entry.len - new_entry.source);

                let left_old = MapEntry {
                    destination: old_entry.destination,
                    source: old_entry.source,
                    len: old_entry.len - overlap_size,
                };
                let right_old = MapEntry {
                    destination: old_entry.destination + left_old.len,
                    source: old_entry.source + left_old.len,
                    len: overlap_size,
                };

                if right_old.len == new_entry.len {
                    // merge right_old with new_entry
                    let merged = MapEntry {
                        destination: right_old.destination,
                        source: new_entry.source,
                        len: overlap_size,
                    };
                    // assign merged to old_entry
                    old_entry.destination = merged.destination;
                    old_entry.source = merged.source;
                    old_entry.len = merged.len;
                    // add left_old to add_to_existing, doesn't need to be in existing because it's strictly lower
                    add_to_existing.push(left_old);
                } else {
                    let left_new = MapEntry {
                        destination: new_entry.destination,
                        source: new_entry.source,
                        len: overlap_size,
                    };
                    let right_new = MapEntry {
                        destination: new_entry.destination + overlap_size,
                        source: new_entry.source + overlap_size,
                        len: new_entry.len - overlap_size,
                    };
                    // merge right_old and left_new
                    let merged = MapEntry {
                        destination: right_old.destination,
                        source: left_new.source,
                        len: overlap_size,
                    };
                    // assign right_old to old_entry
                    old_entry.destination = merged.destination;
                    old_entry.source = merged.source;
                    old_entry.len = merged.len;
                    // add right_new and left_old
                    add_to_existing.push(left_old);
                    // add_to_existing.push(right_new);
                    stack.push(right_new);
                    continue;
                }
            }
        }
        if !has_overlap {
            add_to_existing.push(new_entry.clone());
        }
    }

    existing.append(&mut add_to_existing);
    existing 
        .sort_by(|a, b| a.source.cmp(&b.source));
}

fn create_blocks(lines: &Vec<String>) -> Vec<Vec<MapEntry>> {
    let mut blocks: Vec<Vec<MapEntry>> = Vec::new();
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

        let new_function = parse_map_entry(line);
        block.push(new_function);
    }

    return blocks;
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

    let blocks = create_blocks(lines);

    let mut fs: Vec<MapEntry> = Vec::new();
    for mut block in blocks {
        collapse(&mut fs, &mut block);
    }

    seeds
        .into_iter()
        .map(|source| layer(source, &fs))
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

    // new very large outer
    #[test]
    fn test_collapse_example1 () {
        let file_path = format!("./data/{DAYSTRING}/example1.txt");
        let lines = load_file(&file_path);
        let blocks = create_blocks(&lines);

        let mut fs: Vec<MapEntry> = Vec::new();
        collapse(&mut fs, &mut blocks[0].clone());

        assert_eq!(fs.iter().map(|x| x.to_input_form()).collect::<Vec::<String>>(),
            Vec::from(["52 50 48", "50 98 2"])
        );

        collapse(&mut fs, &mut blocks[1].clone());

        assert_eq!(fs.iter().map(|x| x.to_input_form()).collect::<Vec::<String>>(),
            Vec::from(["39 0 15", "0 15 35", "37 50 2", "54 52 46", "35 98 2"])
        );
    }
}
