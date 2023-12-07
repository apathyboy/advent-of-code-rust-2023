advent_of_code::solution!(7);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard = 0,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn identify_hand(hand: &Vec<u32>) -> Hand {
    let mut cards = hand.clone();
    cards.sort_unstable();
    let mut counts: Vec<u32> = vec![0; 15];
    for card in cards.iter() {
        counts[*card as usize] += 1;
    }
    let mut counts: Vec<(u32, u32)> = counts
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count > 0)
        .map(|(card, count)| (count, card as u32))
        .collect();
    counts.sort_unstable();
    let counts: Vec<u32> = counts.into_iter().map(|(count, _)| count).collect();
    match counts.as_slice() {
        [1, 1, 1, 1, 1] => Hand::HighCard,
        [1, 1, 1, 2] => Hand::OnePair,
        [1, 2, 2] => Hand::TwoPairs,
        [1, 1, 3] => Hand::ThreeOfAKind,
        [2, 3] => Hand::FullHouse,
        [1, 4] => Hand::FourOfAKind,
        [5] => Hand::FiveOfAKind,
        _ => panic!("Invalid hand: {:?}", hand),
    }
}

fn parse_hand(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|card| match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap(),
        })
        .collect()
}

fn identify_hand2(hand: &Vec<u32>) -> Hand {
    let mut cards = hand.clone();
    cards.sort_unstable();
    let mut counts: Vec<u32> = vec![0; 15];
    for card in cards.iter() {
        counts[*card as usize] += 1;
    }
    let mut counts: Vec<(u32, u32)> = counts
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count > 0)
        .map(|(card, count)| (count, card as u32))
        .collect();
    counts.sort_unstable();

    let j_counts = cards.iter().filter(|card| **card == 0).count() as u32;
    let mut counts: Vec<(u32, u32)> = counts
        .into_iter()
        .rev()
        .filter(|(_, card)| *card != 0)
        .collect();
    if counts.is_empty() {
        counts.push((5, 0));
    } else {
        counts[0].0 += j_counts;
    }

    let counts: Vec<u32> = counts.into_iter().rev().map(|(count, _)| count).collect();
    match counts.as_slice() {
        [1, 1, 1, 1, 1] => Hand::HighCard,
        [1, 1, 1, 2] => Hand::OnePair,
        [1, 2, 2] => Hand::TwoPairs,
        [1, 1, 3] => Hand::ThreeOfAKind,
        [2, 3] => Hand::FullHouse,
        [1, 4] => Hand::FourOfAKind,
        [5] => Hand::FiveOfAKind,
        _ => panic!("Invalid hand: {:?}", hand),
    }
}

fn parse_hand2(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|card| match card {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut games: Vec<(Hand, Vec<u32>, u32)> = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let cards: Vec<u32> = parse_hand(parts.0);
            (identify_hand(&cards), cards, parts.1.parse().unwrap())
        })
        .collect();
    games.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => {
            a.1.iter()
                .zip(b.1.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|order| *order != std::cmp::Ordering::Equal)
                .unwrap()
        }
        other => other,
    });
    let winnings = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u32 + 1) * game.2)
        .sum();

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut games: Vec<(Hand, Vec<u32>, u32)> = input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let cards: Vec<u32> = parse_hand2(parts.0);
            (identify_hand2(&cards), cards, parts.1.parse().unwrap())
        })
        .collect();
    games.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => {
            a.1.iter()
                .zip(b.1.iter())
                .map(|(a, b)| a.cmp(b))
                .find(|order| *order != std::cmp::Ordering::Equal)
                .unwrap()
        }
        other => other,
    });
    let winnings = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u32 + 1) * game.2)
        .sum();

    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand2() {
        assert_eq!(parse_hand2("32T3K"), vec![3, 2, 10, 3, 13]);
        assert_eq!(parse_hand2("T55J5"), vec![10, 5, 5, 0, 5]);
        assert_eq!(parse_hand2("KK677"), vec![13, 13, 6, 7, 7]);
        assert_eq!(parse_hand2("KTJJT"), vec![13, 10, 0, 0, 10]);
        assert_eq!(parse_hand2("QQQJA"), vec![12, 12, 12, 0, 14]);
    }

    #[test]
    fn test_identify_hand() {
        assert_eq!(
            identify_hand(&parse_hand("32T3K")),
            Hand::OnePair,
            "One pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("T55J5")),
            Hand::ThreeOfAKind,
            "Three of a kind"
        );
        assert_eq!(
            identify_hand(&parse_hand("KK677")),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("KTJJT")),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("QQQJA")),
            Hand::ThreeOfAKind,
            "Three of a kind"
        );
    }

    #[test]
    fn test_identify_hand2() {
        assert_eq!(
            identify_hand2(&parse_hand2("32T3K")),
            Hand::OnePair,
            "One pair"
        );
        assert_eq!(
            identify_hand2(&parse_hand2("T55J5")),
            Hand::FourOfAKind,
            "Four of a kind"
        );
        assert_eq!(
            identify_hand2(&parse_hand2("KK677")),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand2(&parse_hand2("KTJJT")),
            Hand::FourOfAKind,
            "Two pair"
        );
        assert_eq!(
            identify_hand2(&parse_hand2("QQQJA")),
            Hand::FourOfAKind,
            "Three of a kind"
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
