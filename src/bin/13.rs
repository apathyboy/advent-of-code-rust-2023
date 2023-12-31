use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflections_with_smudge(lines: &[Vec<char>]) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| a.iter().zip(b.iter()).filter(|(a, b)| a != b).count() <= 1)
        .find_map(|((idx_a, _), (idx_b, _))| {
            let prev_lines = lines[..=idx_a].iter().rev();
            let next_lines = lines[idx_b..].iter();

            (prev_lines
                .flatten()
                .zip(next_lines.flatten())
                .filter(|(prev, next)| prev != next)
                .count()
                == 1)
                .then_some(idx_a + 1)
        })
}

fn find_reflections(lines: &[Vec<char>]) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, a), (_, b))| a == b)
        .find_map(|((idx_a, _), (idx_b, _))| {
            let prev_lines = lines[..idx_a].iter().rev();
            let next_lines = lines[idx_b + 1..].iter();

            prev_lines
                .zip(next_lines)
                .all(|(prev, next)| prev == next)
                .then_some(idx_a + 1)
        })
}

fn vertical_reflections(input: &str) -> Option<Reflection> {
    let lines =
        advent_of_code::transpose(input.lines().map(|line| line.chars().collect()).collect());

    find_reflections(&lines).map(Reflection::Vertical)
}

fn horizontal_reflections(input: &str) -> Option<Reflection> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    find_reflections(&lines).map(Reflection::Horizontal)
}

fn reflections(input: &str) -> Option<Reflection> {
    vertical_reflections(input).or_else(|| horizontal_reflections(input))
}

fn vertical_reflections_with_smudge(input: &str) -> Option<Reflection> {
    let lines =
        advent_of_code::transpose(input.lines().map(|line| line.chars().collect()).collect());

    find_reflections_with_smudge(&lines).map(Reflection::Vertical)
}

fn horizontal_reflections_with_smudge(input: &str) -> Option<Reflection> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    find_reflections_with_smudge(&lines).map(Reflection::Horizontal)
}

fn reflections_with_smudge(input: &str) -> Option<Reflection> {
    horizontal_reflections_with_smudge(input).or_else(|| vertical_reflections_with_smudge(input))
}

pub fn part_one(input: &str) -> Option<usize> {
    let (horizontal, vertical) = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .flat_map(reflections)
        .fold((0, 0), |mut acc, item| match item {
            Reflection::Horizontal(len) => {
                acc.0 += len * 100;
                acc
            }
            Reflection::Vertical(len) => {
                acc.1 += len;
                acc
            }
        });

    Some(horizontal + vertical)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (horizontal, vertical) = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .flat_map(reflections_with_smudge)
        .fold((0, 0), |mut acc, item| match item {
            Reflection::Horizontal(len) => {
                acc.0 += len * 100;
                acc
            }
            Reflection::Vertical(len) => {
                acc.1 += len;
                acc
            }
        });

    Some(horizontal + vertical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_fold() {
        let result = horizontal_reflections(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));

        assert_eq!(result, Some(Reflection::Horizontal(4)));
    }

    #[test]
    fn test_horizontal_fold_with_smudge() {
        let result = horizontal_reflections_with_smudge(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        assert_eq!(result, Some(Reflection::Horizontal(3)));
    }

    #[test]
    fn test_horizontal_fold_with_smudge_2() {
        let result = horizontal_reflections_with_smudge(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));

        assert_eq!(result, Some(Reflection::Horizontal(1)));
    }

    #[test]
    fn test_vertical_fold() {
        let result = vertical_reflections(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));

        assert_eq!(result, Some(Reflection::Vertical(5)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
