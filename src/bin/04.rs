use std::collections::HashMap;

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

fn parse_line(line: &str) -> Option<Card> {
    let parts = line
        .split(|c| [':', '|'].contains(&c))
        .skip(1)
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    Some(Card::new(parts[0].clone(), parts[1].clone()))
}

fn collect_cards(map: &mut HashMap<usize, u32>, cards: &Vec<Card>, card_id: usize) {
    let card = cards.get(card_id - 1).unwrap();
    let wins = card.won();

    *map.entry(card_id).or_default() += 1;

    for i in 1..=wins {
        collect_cards(map, cards, i + card_id);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let winnings = input
        .lines()
        .map(|line| parse_line(line).unwrap().score())
        .sum();

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_counts: HashMap<usize, u32> = HashMap::new();
    let cards: Vec<Card> = input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .collect::<Vec<Card>>();

    for (i, _) in cards.iter().enumerate() {
        collect_cards(&mut card_counts, &cards, i + 1);
    }

    Some(card_counts.values().sum())
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
