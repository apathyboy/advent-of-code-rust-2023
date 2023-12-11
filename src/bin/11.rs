use std::{
    cmp::{max, min},
    collections::HashMap,
};

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Element {
    EmptySpace,
    Galaxy,
}

fn find_column_expansions(map: &HashMap<(i32, i32), Element>, width: i32, height: i32) -> Vec<i32> {
    let mut column_expansions: Vec<i32> = Vec::new();

    for x in 0..width {
        let mut empty_column: bool = true;
        for y in 0..height {
            let element = map.get(&(x, y)).unwrap();
            if element == &Element::Galaxy {
                empty_column = false;
            }
        }
        if empty_column {
            column_expansions.push(x);
        }
    }

    column_expansions
}

fn find_sum_distances(
    galaxies: &[(i32, i32)],
    column_expansions: &[i32],
    row_expansions: &[i32],
    growth_factor: u64,
) -> Option<u64> {
    let sum_distances = galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .filter_map(|(a, b)| {
            if a == b {
                None
            } else if (b.0 < a.0) || (a.0 == b.0 && b.1 < a.1) {
                Some((b, a))
            } else {
                Some((a, b))
            }
        })
        .unique()
        .map(|(a, b)| {
            let x_diff = (a.0 - b.0).unsigned_abs() as u64;
            let y_diff = (a.1 - b.1).unsigned_abs() as u64;

            let x_expansion = column_expansions
                .iter()
                .filter(|&x| *x < max(a.0, b.0) && *x > min(a.0, b.0))
                .count() as u64;
            let y_expansion = row_expansions
                .iter()
                .filter(|&y| *y < max(a.1, b.1) && *y > min(a.1, b.1))
                .count() as u64;

            x_diff
                + y_diff
                + ((x_expansion * growth_factor) - x_expansion)
                + ((y_expansion * growth_factor) - y_expansion)
        })
        .sum();

    Some(sum_distances)
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<i32>, Vec<i32>) {
    let mut map: HashMap<(i32, i32), Element> = HashMap::new();
    let mut galaxies: Vec<(i32, i32)> = Vec::new();
    let mut row_expansions: Vec<i32> = Vec::new();

    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().chars().count() as i32;

    for (y, line) in input.lines().enumerate() {
        let mut is_row_expansion: bool = true;

        for (x, c) in line.chars().enumerate() {
            let element = match c {
                '.' => Element::EmptySpace,
                '#' => {
                    galaxies.push((x as i32, y as i32));
                    is_row_expansion = false;
                    Element::Galaxy
                }
                _ => panic!("Unknown element"),
            };

            map.insert((x as i32, y as i32), element);
        }

        if is_row_expansion {
            row_expansions.push(y as i32);
        }
    }

    let column_expansions = find_column_expansions(&map, width, height);

    (galaxies, column_expansions, row_expansions)
}

fn part_one(input: &str) -> Option<u64> {
    let (galaxies, column_expansions, row_expansions) = parse_input(input);

    find_sum_distances(&galaxies, &column_expansions, &row_expansions, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (galaxies, column_expansions, row_expansions) = parse_input(input);

    find_sum_distances(&galaxies, &column_expansions, &row_expansions, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let (galaxies, column_expansions, row_expansions) =
            parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = find_sum_distances(&galaxies, &column_expansions, &row_expansions, 10);
        assert_eq!(result, Some(1030));
        let result = find_sum_distances(&galaxies, &column_expansions, &row_expansions, 100);
        assert_eq!(result, Some(8410));
    }
}
