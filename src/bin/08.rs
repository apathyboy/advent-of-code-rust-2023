use num_integer::lcm;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let directions = input.lines().next().unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in input.lines().skip(2) {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    (directions, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (directions, map) = parse_input(input);

    let mut acc = 0;
    let mut current = "AAA";

    for c in directions.chars().cycle() {
        let (left, right) = map.get(current).unwrap();

        if c == 'L' {
            current = left;
        } else {
            current = right;
        }

        acc += 1;

        if current == "ZZZ" {
            break;
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (directions, map) = parse_input(input);

    let current = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect::<Vec<&str>>();

    let searches: Vec<u64> = current
        .iter()
        .map(|k| {
            let mut acc = 0;
            let mut cur = *k;

            for c in directions.chars().cycle() {
                let (left, right) = map.get(cur).unwrap();

                if c == 'L' {
                    cur = left;
                } else {
                    cur = right;
                }

                acc += 1;

                if cur.ends_with('Z') {
                    break;
                }
            }

            acc
        })
        .collect();

    searches.iter().copied().reduce(lcm)
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
