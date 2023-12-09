advent_of_code::solution!(9);

fn parse_histories(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(advent_of_code::parse_space_separated)
        .collect::<Vec<_>>()
}

fn process_history(history: &[i32]) -> Option<i32> {
    let mut extrapolated = history.last().copied().unwrap();
    let mut history = history.to_vec();

    while !history.iter().all(|&x| x == 0) {
        history = history
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect::<Vec<_>>();

        extrapolated += history.last().copied().unwrap();
    }

    Some(extrapolated)
}

pub fn part_one(input: &str) -> Option<i32> {
    parse_histories(input)
        .iter()
        .map(|history| process_history(history))
        .sum()
}

pub fn part_two(input: &str) -> Option<i32> {
    parse_histories(input)
        .iter()
        .map(|history| {
            let reversed = history.iter().rev().cloned().collect::<Vec<_>>();
            process_history(&reversed)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_history() {
        assert_eq!(process_history(&[0, 3, 6, 9, 12, 15]), Some(18));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
