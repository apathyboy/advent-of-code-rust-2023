use num_integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (&str, HashMap<&str, [&str; 2]>) {
    let directions = input.lines().next().unwrap();
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    for line in input.lines().skip(2) {
        map.insert(&line[0..3], [&line[7..10], &line[12..15]]);
    }

    (directions, map)
}

fn traverse(
    map: &HashMap<&str, [&str; 2]>,
    directions: &str,
    start: &str,
    f: fn(&str) -> bool,
) -> Option<u64> {
    let mut acc = 0;
    let mut current = start;

    for c in directions.chars().cycle() {
        if c == 'L' {
            current = map.get(current).unwrap()[0];
        } else {
            current = map.get(current).unwrap()[1];
        }

        acc += 1;

        if f(current) {
            break;
        }
    }

    Some(acc)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (directions, map) = parse_input(input);

    traverse(&map, directions, "AAA", |c| c == "ZZZ")
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, map) = parse_input(input);

    map.keys()
        .filter_map(|k| match k.ends_with('A') {
            true => traverse(&map, directions, k, |c| c.ends_with('Z')),
            false => None,
        })
        .reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
