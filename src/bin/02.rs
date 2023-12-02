advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
pub struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn power(&self) -> Option<u32> {
        Some(self.red * self.green * self.blue)
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

pub fn parse_line(line: &str) -> Option<Game> {
    let mut game = Game::new(0, 0, 0);

    for val in line.split(|c| [':', ';', ','].contains(&c)).skip(1) {
        let (value, color) = val.trim().split_once(' ').expect("Could not split on ' '");
        let value = value.parse::<u32>().expect("Could not parse value");

        match color {
            "red" => game.red = std::cmp::max(value, game.red),
            "green" => game.green = std::cmp::max(value, game.green),
            "blue" => game.blue = std::cmp::max(value, game.blue),
            _ => (),
        }
    }

    Some(game)
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum_of_ids = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| match parse_line(line)?.is_valid() {
            true => Some((i + 1) as u32),
            false => None,
        })
        .sum();

    Some(sum_of_ids)
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(|line| parse_line(line)?.power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result =
            parse_line("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(result, Some(Game::new(14, 3, 15)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
