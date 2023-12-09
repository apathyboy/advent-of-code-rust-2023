advent_of_code::solution!(9);

fn parse_histories(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(advent_of_code::parse_space_separated)
        .collect::<Vec<_>>()
}

fn process_history(history: &[u32]) -> Option<u32> {
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_histories(input)
        .iter()
        .map(|history| process_history(history))
        .sum()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
