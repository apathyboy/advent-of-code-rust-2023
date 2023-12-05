use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Debug)]
struct AlmanacMap {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl AlmanacMap {
    fn new(dest_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            dest_range_start,
            source_range_start,
            range_length,
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
            let parts: Vec<u64> = advent_of_code::parse_space_separated(line);
            current_maps.push(AlmanacMap::new(parts[0], parts[1], parts[2]));
        }
    }

    almanac_maps.push(current_maps);

    almanac_maps
}

fn parse_seeds(input: &str) -> Vec<u64> {
    advent_of_code::parse_space_separated::<u64>(&input[6..])
}

fn parse_seed_ranges(input: &str) -> Vec<(u64, u64)> {
    let seed_ranges = advent_of_code::parse_space_separated::<u64>(&input[6..]);

    seed_ranges
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect()
}

fn process_seed_to_location(seed: u64, almanac_maps: &Vec<Vec<AlmanacMap>>) -> u64 {
    almanac_maps.iter().fold(seed, |seed_test, almanac_map| {
        for map in almanac_map {
            if seed_test >= map.source_range_start
                && seed_test < map.source_range_start + map.range_length
            {
                return map.dest_range_start + (seed_test - map.source_range_start);
            }
        }

        seed_test
    })
}

fn process_location_to_seed(location: u64, almanac_maps: &Vec<Vec<AlmanacMap>>) -> u64 {
    almanac_maps
        .iter()
        .rev()
        .fold(location, |location, almanac_map| {
            for map in almanac_map {
                if location >= map.dest_range_start
                    && location < map.dest_range_start + map.range_length
                {
                    return map.source_range_start + (location - map.dest_range_start);
                }
            }

            location
        })
}

fn seed_in_ranges(seed: u64, seed_ranges: &Vec<(u64, u64)>) -> bool {
    seed_ranges
        .iter()
        .any(|(start, length)| seed >= *start && seed <= *start + *length)
}

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    seeds
        .iter()
        .map(|seed| process_seed_to_location(*seed, &almanac_maps))
        .min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let seed_ranges = parse_seed_ranges(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    (0..u64::MAX)
        .map(|location| (location, process_location_to_seed(location, &almanac_maps)))
        .filter(|(_, seed)| seed_in_ranges(*seed, &seed_ranges))
        .map(|(location, _)| location)
        .next()
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
        assert_eq!(seeds, vec![79_u64, 14, 55, 13]);
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
