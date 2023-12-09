advent_of_code::solution!(9);

fn parse_histories(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(advent_of_code::parse_space_separated)
        .collect::<Vec<_>>()
}

fn process_history(history: &[i32]) -> Option<i32> {
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    let mut current_sequence = Vec::new();

    sequences.push(history.to_vec());

    loop {
        for slice in sequences.last().unwrap().windows(2) {
            let (a, b) = (slice[0], slice[1]);

            current_sequence.push(b - a);
        }

        sequences.push(current_sequence.clone());

        if current_sequence.iter().sum::<i32>() == 0 {
            break;
        }

        current_sequence.clear();
    }

    let mut last: i32 = 0;

    for sequence in sequences.iter().rev() {
        last += sequence.last().unwrap();
    }

    Some(last)
}

fn process_history2(history: &[i32]) -> Option<i32> {
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    let mut current_sequence = Vec::new();
    sequences.push(history.to_vec());

    loop {
        for slice in sequences.last().unwrap().windows(2) {
            let (a, b) = (slice[0], slice[1]);

            current_sequence.push(b - a);
        }

        sequences.push(current_sequence.clone());

        if current_sequence.iter().sum::<i32>() == 0 {
            break;
        }

        current_sequence.clear();
    }

    let mut first: i32 = 0;

    for sequence in sequences.iter().rev() {
        first = sequence.first().unwrap() - first;
    }

    Some(first)
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
        .map(|history| process_history2(history))
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
    fn test_process_history2() {
        assert_eq!(process_history2(&[10, 13, 16, 21, 30, 45]), Some(5));
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
