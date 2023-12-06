use itertools::Itertools;
use range_ext::intersect::*;
use std::{collections::VecDeque, ops::Range};
advent_of_code::solution!(5);

#[derive(Debug)]
struct AlmanacMap {
    source_range: Range<i64>,
    diff: i64,
}

impl AlmanacMap {
    fn new(dest_range_start: i64, source_range_start: i64, range_length: i64) -> Self {
        Self {
            source_range: Range {
                start: source_range_start,
                end: source_range_start + range_length,
            },
            diff: dest_range_start - source_range_start,
        }
    }
}

fn parse_almanac_maps(input: &str) -> Vec<Vec<AlmanacMap>> {
    let mut almanac_maps = Vec::new();
    let mut current_maps = Vec::new();

    for line in input.lines().skip(2) {
        if line.trim().is_empty() {
            almanac_maps.push(current_maps);
            current_maps = Vec::new();
        } else if !line.ends_with("map:") {
            let parts: Vec<i64> = advent_of_code::parse_space_separated(line);
            current_maps.push(AlmanacMap::new(parts[0], parts[1], parts[2]));
        }
    }

    almanac_maps.push(current_maps);

    almanac_maps
}

fn parse_seeds(input: &str) -> Vec<i64> {
    advent_of_code::parse_space_separated(&input[6..])
}

fn parse_seed_ranges(input: &str) -> Vec<Range<i64>> {
    let seed_ranges = advent_of_code::parse_space_separated::<i64>(&input[6..]);

    seed_ranges
        .chunks(2)
        .map(|chunk| Range {
            start: chunk[0],
            end: chunk[0] + chunk[1],
        })
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect()
}

fn process_seed_to_location(seed: i64, almanac_maps: &Vec<Vec<AlmanacMap>>) -> i64 {
    almanac_maps.iter().fold(seed, |seed_test, almanac_map| {
        match almanac_map
            .iter()
            .find(|map| map.source_range.contains(&seed_test))
        {
            Some(map) => return seed_test + map.diff,
            None => (),
        }

        seed_test
    })
}

fn ranged_explore(
    seed_ranges: &Vec<Range<i64>>,
    almanac_maps: &Vec<Vec<AlmanacMap>>,
) -> Option<i64> {
    let mut ranges: VecDeque<Range<i64>> = seed_ranges.iter().cloned().collect();
    let mut next_ranges: VecDeque<Range<i64>> = VecDeque::new();

    for stage in almanac_maps {
        while let Some(mut seeds) = ranges.pop_front() {
            for map in stage.iter() {
                match seeds.intersect_ext(&map.source_range) {
                    IntersectionExt::LessOverlap => {
                        next_ranges
                            .push_back(map.source_range.start + map.diff..seeds.end + map.diff);
                        seeds = seeds.start..map.source_range.start;
                    }
                    IntersectionExt::GreaterOverlap => {
                        next_ranges
                            .push_back(seeds.start + map.diff..map.source_range.end + map.diff);
                        seeds = map.source_range.end..seeds.end;
                    }
                    IntersectionExt::Within | IntersectionExt::Same => {
                        next_ranges.push_back(seeds.start + map.diff..seeds.end + map.diff);
                        seeds = 0..0;
                        //continue 'seeds;
                    }
                    IntersectionExt::Over => {
                        next_ranges.push_back(
                            map.source_range.start + map.diff..map.source_range.end + map.diff,
                        );
                        ranges.push_front(seeds.start..map.source_range.start);
                        ranges.push_front(map.source_range.end..seeds.end);
                        seeds = 0..0;
                        //continue 'seeds;
                    }
                    _ => {}
                };

                if seeds.is_empty() {
                    break;
                }
            }

            if seeds.end > seeds.start {
                next_ranges.push_back(seeds);
            }
        }

        ranges = next_ranges;
        next_ranges = VecDeque::new();
    }

    ranges.iter().map(|r| r.start).min()
}

pub fn part_one(input: &str) -> Option<i64> {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    seeds
        .iter()
        .map(|seed| process_seed_to_location(*seed, &almanac_maps))
        .min()
}

pub fn part_two(input: &str) -> Option<i64> {
    let seed_ranges = parse_seed_ranges(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    ranged_explore(&seed_ranges, &almanac_maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let seeds = parse_seeds(
            &advent_of_code::template::read_file("examples", DAY)
                .lines()
                .next()
                .unwrap(),
        );
        assert_eq!(seeds, vec![79_i64, 14, 55, 13]);
    }

    #[test]
    fn test_parse_almanac_maps() {
        let result = parse_almanac_maps(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
