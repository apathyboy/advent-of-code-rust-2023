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

fn process_seed(seed: u64, almanac_maps: &Vec<Vec<AlmanacMap>>) -> u64 {
    let mut seed = seed;

    for almanac_map in almanac_maps {
        for map in almanac_map {
            if seed >= map.source_range_start && seed < map.source_range_start + map.range_length {
                seed = map.dest_range_start + (seed - map.source_range_start);
                break;
            }
        }
    }

    seed
}

pub fn part_one(input: &str) -> Option<u64> {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    let mut min_location = u64::MAX;

    for mut seed in seeds {
        seed = process_seed(seed, &almanac_maps);

        if seed < min_location {
            min_location = seed;
        }
    }

    Some(min_location)
}

pub fn part_two(input: &str) -> Option<u64> {
    let seeds = parse_seeds(input.lines().next().unwrap());
    let almanac_maps = parse_almanac_maps(input);

    let mut min_location = u64::MAX;

    for chunk in seeds.chunks(2) {
        for mut seed in chunk[0]..(chunk[0] + chunk[1]) {
            seed = process_seed(seed, &almanac_maps);

            if seed < min_location {
                min_location = seed;
            }
        }
    }

    Some(min_location)
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
