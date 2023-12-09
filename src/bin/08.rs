use num_integer::lcm;
use rayon::prelude::*;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn hash_string(s: &str) -> u32 {
    let bytes = s.as_bytes();
    if bytes.len() != 3 {
        panic!("Invalid string length");
    }

    let mut result = 0;
    result |= (bytes[0] as u32) << 16;
    result |= (bytes[1] as u32) << 8;
    result |= bytes[2] as u32;

    result
}

fn ends_with(hashed_value: u32, val: u8) -> bool {
    (hashed_value & 0xFF) as u8 == val
}

fn traverse(
    map: &HashMap<u32, [u32; 2]>,
    directions: &str,
    start: u32,
    f: impl Fn(u32) -> bool,
) -> Option<u64> {
    let mut acc = 0;
    let mut current = start;

    for c in directions.chars().cycle() {
        if c == 'L' {
            current = map.get(&current).unwrap()[0];
        } else {
            current = map.get(&current).unwrap()[1];
        }

        acc += 1;

        if f(current) {
            break;
        }
    }

    Some(acc)
}

fn parse_input(input: &str) -> (&str, HashMap<u32, [u32; 2]>) {
    let directions = input.lines().next().unwrap();
    let mut map: HashMap<u32, [u32; 2]> = HashMap::new();

    for line in input.lines().skip(2) {
        map.insert(
            hash_string(&line[0..3]),
            [hash_string(&line[7..10]), hash_string(&line[12..15])],
        );
    }

    (directions, map)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (directions, map) = parse_input(input);

    traverse(&map, directions, hash_string("AAA"), |c| {
        c == hash_string("ZZZ")
    })
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, map) = parse_input(input);

    let keys: Vec<&u32> = map.keys().collect();

    let tmp = keys
        .into_par_iter()
        .filter_map(|k| match ends_with(*k, b'A') {
            true => traverse(&map, directions, *k, |c| ends_with(c, b'Z')),
            false => None,
        })
        .collect::<Vec<u64>>();

    tmp.into_iter().reduce(lcm)
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
