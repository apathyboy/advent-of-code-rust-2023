advent_of_code::solution!(1);

fn calculate_calibration_value_pt1(input: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::<u32>::new();

    for (_index, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            nums.push(c.to_digit(10).unwrap());
        }
    }

    let first_digit = nums.first().unwrap();
    let last_digit = nums.last().unwrap();

    first_digit * 10 + last_digit
}

fn calculate_calibration_value_pt2(input: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::<u32>::new();
    let number_words = [
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for (_index, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            nums.push(c.to_digit(10).unwrap());
        } else {
            for (word, digit) in number_words.iter() {
                if input[_index..].starts_with(word) {
                    nums.push(digit.parse::<u32>().unwrap());
                }
            }
        }
    }

    let first_digit = nums.first().unwrap();
    let last_digit = nums.last().unwrap();

    first_digit * 10 + last_digit
}

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input.lines().map(calculate_calibration_value_pt1).sum();

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer: u32 = input.lines().map(calculate_calibration_value_pt2).sum();

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
