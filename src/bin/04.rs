advent_of_code::solution!(4);

fn parse_line(line: &str) -> Option<usize> {
    let parts: Vec<Vec<u32>> = line
        .split(|c| [':', '|'].contains(&c))
        .skip(1)
        .map(advent_of_code::parse_space_separated)
        .collect();

    Some(parts[0].iter().filter(|&x| parts[1].contains(x)).count())
}

fn score(wins: usize) -> u32 {
    match wins.checked_sub(1) {
        Some(num) => 2_u32.pow(num as u32),
        None => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(parse_line).map(score).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let cards: Vec<usize> = input.lines().filter_map(parse_line).collect();
    let copies = vec![1_usize; cards.len()];

    let total_cards: usize = cards
        .iter()
        .enumerate()
        .fold(copies, |mut acc, (i, card)| {
            for j in 1..=*card {
                acc[i + j] += acc[i];
            }
            acc
        })
        .iter()
        .sum();

    Some(total_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let wins = parse_line(line).unwrap();
        assert_eq!(wins, 4);
    }

    #[test]
    fn test_score() {
        assert_eq!(score(4), 8);
        assert_eq!(score(2), 2);
        assert_eq!(score(1), 1);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
