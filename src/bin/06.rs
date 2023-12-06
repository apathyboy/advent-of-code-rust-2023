advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

fn calculate_distance(time: u64, race_time: u64) -> u64 {
    (race_time - time) * time
}

fn find_partition_point(start: u64, end: u64, race: &Race) -> u64 {
    let mut left = start;
    let mut right = end;

    while left < right {
        let mid = left + (right - left) / 2;
        if calculate_distance(mid, race.time) > race.distance {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn count_potential_wins(race: &Race) -> u64 {
    // Check the left "half" (partition) for the point when race wins start
    let partition_point = find_partition_point(0, race.time / 2, race);

    (race.time - (partition_point * 2)) + 1
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
        let race1 = count_potential_wins(&Race {
            time: 7,
            distance: 9,
        });
        let race2 = count_potential_wins(&Race {
            time: 15,
            distance: 40,
        });
        let race3 = count_potential_wins(&Race {
            time: 30,
            distance: 200,
        });
        assert_eq!(race1, 4);
        assert_eq!(race2, 8);
        assert_eq!(race3, 9);
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
