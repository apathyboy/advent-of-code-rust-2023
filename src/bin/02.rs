use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(2);

fn game_number(input: &str) -> Option<u32> {
    let re = Regex::new(r"Game (\d+)").unwrap();
    let caps = re.captures(input)?;
    let game_number = caps.get(1)?.as_str().parse::<u32>().ok()?;
    Some(game_number)
}

fn max_color_values(input: &str) -> (u32, u32, u32) {
    let re = Regex::new(r"(\d+) (green|red|blue)").unwrap();
    let mut max_values: HashMap<&str, u32> = HashMap::new();

    for (_, [value, color]) in re.captures_iter(input).map(|c| c.extract()) {
        let value = value.parse::<u32>().unwrap();

        let entry = max_values.entry(color).or_insert(0);
        if value > *entry {
            *entry = value;
        }
    }

    let max_red = *max_values.get("red").unwrap();
    let max_green = *max_values.get("green").unwrap();
    let max_blue = *max_values.get("blue").unwrap();

    (max_red, max_green, max_blue)
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum_of_ids = input
        .lines()
        .filter_map(|line| {
            let game_number = game_number(line);
            let (max_red, max_green, max_blue) = max_color_values(line);

            match max_red <= 12 && max_green <= 13 && max_blue <= 14 {
                true => game_number,
                false => None,
            }
        })
        .sum();

    Some(sum_of_ids)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum_of_powers = input
        .lines()
        .map(|line| {
            let (max_red, max_green, max_blue) = max_color_values(line);

            max_red * max_green * max_blue
        })
        .sum();

    Some(sum_of_powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_number() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = game_number(input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_max_values() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let (red, green, blue) = max_color_values(input);
        assert_eq!(red, 14);
        assert_eq!(green, 3);
        assert_eq!(blue, 15);
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
