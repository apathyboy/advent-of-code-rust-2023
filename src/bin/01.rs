advent_of_code::solution!(1);

fn calculate_calibration_value(input: &str) -> Option<u32> {
    let nums: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let first_digit = nums.first().expect("Expect at least one valid digit");
    let last_digit = nums.last().expect("Expect at least one valid digit");

    Some(first_digit * 10 + last_digit)
}

fn calculate_real_calibration_value(input: &str) -> Option<u32> {
    let number_words = [
        ("zero", 0_u32),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let nums: Vec<u32> = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.is_ascii_digit() {
                return c.to_digit(10);
            }

            number_words.iter().find_map(|(word, digit)| {
                if input[i..].starts_with(word) {
                    return Some(*digit);
                }
                None
            })
        })
        .collect();

    let first_digit = nums.first().expect("Expect at least one valid digit");
    let last_digit = nums.last().expect("Expect at least one valid digit");

    Some(first_digit * 10 + last_digit)
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(calculate_calibration_value).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(calculate_real_calibration_value).sum()
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
