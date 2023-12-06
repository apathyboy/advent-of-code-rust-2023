advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

fn count_potential_wins(race: &Race) -> u64 {
    (0..=race.time)
        .map(|t| {
            let remaining = race.time - t;
            remaining * t
        })
        .filter(|d| *d > race.distance)
        .count() as u64
}

fn parse_races(input: &str) -> Vec<Race> {
    let parts: Vec<Vec<u64>> = input
        .lines()
        .map(|s| advent_of_code::parse_space_separated(&s[11..]))
        .collect();

    parts[0]
        .iter()
        .zip(parts[1].iter())
        .map(|(a, b)| Race {
            time: *a,
            distance: *b,
        })
        .collect()
}

fn parse_race(input: &str) -> Race {
    let parts: Vec<u64> = input
        .lines()
        .map(|s| {
            s[11..]
                .split_whitespace()
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect();

    Race {
        time: parts[0],
        distance: parts[1],
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let margin_of_error = parse_races(input)
        .iter()
        .map(count_potential_wins)
        .product();

    Some(margin_of_error)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(count_potential_wins(&parse_race(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_race() {
        let result = parse_race(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Race {
                time: 71530,
                distance: 940200
            }
        );
    }

    #[test]
    fn test_count_potential_wins() {
        let race = Race {
            time: 7,
            distance: 9,
        };
        let result = count_potential_wins(&race);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_races() {
        let result = parse_races(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
