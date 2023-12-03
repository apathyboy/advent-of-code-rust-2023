use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut engine_schematic_total: u32 = 0;
    let map: Vec<&str> = input.lines().collect::<Vec<&str>>();

    for (i, line) in map.iter().enumerate() {
        let mut potential_part_number: String = String::new();
        let mut is_part_number = false;

        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                potential_part_number.push(c);

                if is_part_number {
                    continue;
                }

                // upper left
                if i > 0 && j > 0 {
                    if !map[i - 1].as_bytes()[j - 1].is_ascii_digit()
                        && map[i - 1].as_bytes()[j - 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // left
                if i > 0 {
                    if !map[i - 1].as_bytes()[j].is_ascii_digit()
                        && map[i - 1].as_bytes()[j] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // lower left
                if i > 0 && j < map.len() - 1 {
                    if !map[i - 1].as_bytes()[j + 1].is_ascii_digit()
                        && map[i - 1].as_bytes()[j + 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // upper
                if j > 0 {
                    if !map[i].as_bytes()[j - 1].is_ascii_digit()
                        && map[i].as_bytes()[j - 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // lower
                if j < map.len() - 1 {
                    if !map[i].as_bytes()[j + 1].is_ascii_digit()
                        && map[i].as_bytes()[j + 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // upper right
                if i < line.len() - 1 && j > 0 {
                    if !map[i + 1].as_bytes()[j - 1].is_ascii_digit()
                        && map[i + 1].as_bytes()[j - 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // right
                if i < line.len() - 1 {
                    if !map[i + 1].as_bytes()[j].is_ascii_digit()
                        && map[i + 1].as_bytes()[j] as char != '.'
                    {
                        is_part_number = true;
                    }
                }

                // lower right
                if i < line.len() - 1 && j < map.len() - 1 {
                    if !map[i + 1].as_bytes()[j + 1].is_ascii_digit()
                        && map[i + 1].as_bytes()[j + 1] as char != '.'
                    {
                        is_part_number = true;
                    }
                }
            } else {
                if is_part_number && potential_part_number.len() > 0 {
                    let num = potential_part_number.parse::<u32>().unwrap();
                    engine_schematic_total += num;
                }
                potential_part_number = String::new();
                is_part_number = false;
            }
        }

        if is_part_number && potential_part_number.len() > 0 {
            let num = potential_part_number.parse::<u32>().unwrap();
            engine_schematic_total += num;
        }
    }

    Some(engine_schematic_total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut gears: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    for (i, line) in map.iter().enumerate() {
        let mut potential_part_number: String = String::new();
        let mut found_gears: Vec<(usize, usize)> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                potential_part_number.push(c);

                // upper left
                if i > 0 && j > 0 {
                    if map[i - 1].as_bytes()[j - 1] as char == '*' {
                        found_gears.push((i - 1, j - 1));
                    }
                }

                // left
                if i > 0 {
                    if map[i - 1].as_bytes()[j] as char == '*' {
                        found_gears.push((i - 1, j));
                    }
                }

                // lower left
                if i > 0 && j < map.len() - 1 {
                    if map[i - 1].as_bytes()[j + 1] as char == '*' {
                        found_gears.push((i - 1, j + 1));
                    }
                }

                // upper
                if j > 0 {
                    if map[i].as_bytes()[j - 1] as char == '*' {
                        found_gears.push((i, j - 1));
                    }
                }

                // lower
                if j < map.len() - 1 {
                    if map[i].as_bytes()[j + 1] as char == '*' {
                        found_gears.push((i, j + 1));
                    }
                }

                // upper right
                if i < line.len() - 1 && j > 0 {
                    if map[i + 1].as_bytes()[j - 1] as char == '*' {
                        found_gears.push((i + 1, j - 1));
                    }
                }

                // right
                if i < line.len() - 1 {
                    if map[i + 1].as_bytes()[j] as char == '*' {
                        found_gears.push((i + 1, j));
                    }
                }

                // lower right
                if i < line.len() - 1 && j < map.len() - 1 {
                    if map[i + 1].as_bytes()[j + 1] as char == '*' {
                        found_gears.push((i + 1, j + 1));
                    }
                }
            } else {
                if found_gears.len() > 0 && potential_part_number.len() > 0 {
                    let num = potential_part_number.parse::<u64>().unwrap();
                    for gear in found_gears.iter().unique() {
                        gears.entry(*gear).or_insert(Vec::new()).push(num);
                    }
                }
                potential_part_number = String::new();
                found_gears = Vec::new();
            }
        }

        if found_gears.len() > 0 && potential_part_number.len() > 0 {
            let num = potential_part_number.parse::<u64>().unwrap();
            for gear in found_gears.iter().unique() {
                gears.entry(*gear).or_insert(Vec::new()).push(num);
            }
        }
    }

    Some(
        gears
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0] * v[1])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
