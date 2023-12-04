advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(winning: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self { winning, numbers }
    }

    fn won(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn score(&self) -> u32 {
        if self.won() == 0 {
            return 0;
        }

        2_u32.pow((self.won() - 1) as u32)
    }
}

fn parse_string_of_u32(s: &str) -> Vec<u32> {
    s.split_whitespace()
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}

fn parse_line(line: &str) -> Option<Card> {
    let parts = line
        .split(|c| [':', '|'].contains(&c))
        .skip(1)
        .map(parse_string_of_u32)
        .collect::<Vec<Vec<u32>>>();

    Some(Card::new(parts[0].clone(), parts[1].clone()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let winnings = input
        .lines()
        .filter_map(|line| parse_line(line))
        .map(|card| card.score())
        .sum();

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input
        .lines()
        .filter_map(|line| parse_line(line))
        .collect::<Vec<Card>>();

    let mut copies = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        for j in 1..=card.won() {
            copies[i + j] += copies[i];
        }
    }

    Some(copies.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_line(line).unwrap();
        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_score() {
        let card = Card::new(vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.won(), 4);
        assert_eq!(card.score(), 8);
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
