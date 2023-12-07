use std::cmp::Ordering;

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

#[derive(Debug, PartialEq, Eq)]
struct Game {
    hand: Hand,
    cards: Vec<u32>,
    bid: u32,
}

impl Game {
    fn new(hand: Hand, cards: Vec<u32>, bid: u32) -> Self {
        Self { hand, cards, bid }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.cmp(&other.hand) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(a, b)| match a.cmp(b) {
                    Ordering::Equal => None,
                    non_equal => Some(non_equal),
                })
                .unwrap(),
            other => other,
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn identify_hand(hand: &Vec<u32>) -> Hand {
    let mut counts: Vec<u32> = vec![0; 15];
    for card in hand {
        counts[*card as usize] += 1;
    }

    let joker_counts = counts.remove(0);

    counts.retain(|count| *count > 0);
    counts.sort_unstable();

    if joker_counts > 0 {
        if let Some(last) = counts.last_mut() {
            *last += joker_counts; // Update the last element
        }
    }

    match counts.as_slice() {
        [1, 1, 1, 1, 1] => Hand::HighCard,
        [1, 1, 1, 2] => Hand::OnePair,
        [1, 2, 2] => Hand::TwoPairs,
        [1, 1, 3] => Hand::ThreeOfAKind,
        [2, 3] => Hand::FullHouse,
        [1, 4] => Hand::FourOfAKind,
        [5] => Hand::FiveOfAKind,
        [] => Hand::FiveOfAKind,
        _ => panic!("Invalid hand: {:?}", hand),
    }
}

fn parse_hand(input: &str, use_jokers: bool) -> Vec<u32> {
    input
        .chars()
        .map(|card| match card {
            'T' => 10,
            'J' => {
                if use_jokers {
                    0
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap(),
        })
        .collect()
}

fn parse_game(input: &str, use_jokers: bool) -> Game {
    let (hand, bid) = input.split_once(' ').unwrap();
    let hand: Vec<u32> = parse_hand(hand, use_jokers);
    Game::new(identify_hand(&hand), hand, bid.parse().unwrap())
}

fn play_games(input: &str, use_jokers: bool) -> Option<u32> {
    let mut games: Vec<Game> = input
        .lines()
        .map(|line| parse_game(line, use_jokers))
        .collect();

    games.sort_unstable();

    let winnings = games
        .iter()
        .enumerate()
        .map(|(i, game)| (i as u32 + 1) * game.bid)
        .sum();

    Some(winnings)
}

pub fn part_one(input: &str) -> Option<u32> {
    play_games(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    play_games(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand_with_jokers() {
        assert_eq!(parse_hand("32T3K", true), vec![3, 2, 10, 3, 13]);
        assert_eq!(parse_hand("T55J5", true), vec![10, 5, 5, 0, 5]);
        assert_eq!(parse_hand("KK677", true), vec![13, 13, 6, 7, 7]);
        assert_eq!(parse_hand("KTJJT", true), vec![13, 10, 0, 0, 10]);
        assert_eq!(parse_hand("QQQJA", true), vec![12, 12, 12, 0, 14]);
    }

    #[test]
    fn test_identify_hand_with_jokers() {
        assert_eq!(
            identify_hand(&parse_hand("32T3K", false)),
            Hand::OnePair,
            "One pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("T55J5", false)),
            Hand::ThreeOfAKind,
            "Three of a kind"
        );
        assert_eq!(
            identify_hand(&parse_hand("KK677", false)),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("KTJJT", false)),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("QQQJA", false)),
            Hand::ThreeOfAKind,
            "Three of a kind"
        );
    }

    #[test]
    fn test_identify_hand2() {
        assert_eq!(
            identify_hand(&parse_hand("32T3K", true)),
            Hand::OnePair,
            "One pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("T55J5", true)),
            Hand::FourOfAKind,
            "Four of a kind"
        );
        assert_eq!(
            identify_hand(&parse_hand("KK677", true)),
            Hand::TwoPairs,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("KTJJT", true)),
            Hand::FourOfAKind,
            "Two pair"
        );
        assert_eq!(
            identify_hand(&parse_hand("QQQJA", true)),
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
