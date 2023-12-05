use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(3);

fn check_for_symbol(map: &Vec<&str>, i: i32, j: i32) -> bool {
    let positions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for (x, y) in positions.iter() {
        let (x, y) = (j as i32 + x, i as i32 + y);

        if x >= 0 && y >= 0 && x < (map.len() - 1) as i32 && y < (map.len() - 1) as i32 {
            if !map[y as usize].as_bytes()[x as usize].is_ascii_digit()
                && map[y as usize].as_bytes()[x as usize] as char != '.'
            {
                return true;
            }
        }
    }

    false
}

fn find_gears(map: &Vec<&str>, i: i32, j: i32, found_gears: &mut Vec<(usize, usize)>) {
    let positions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    for (x, y) in positions.iter() {
        let (x, y) = (j as i32 + x, i as i32 + y);

        if x >= 0 && y >= 0 && x < (map.len() - 1) as i32 && y < (map.len() - 1) as i32 {
            if map[y as usize].as_bytes()[x as usize] as char == '*' {
                found_gears.push((y as usize, x as usize));
            }
        }
    }
}

fn part_one(input: &str) -> Option<u32> {
    let mut engine_schematic_total: u32 = 0;
    let map: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let mut potential_part_number: String = String::new();
    let mut is_part_number = false;

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                potential_part_number.push(c);

                if !is_part_number {
                    is_part_number = check_for_symbol(&map, i as i32, j as i32);
                }
            }
            if !c.is_ascii_digit() || j == line.len() - 1 {
                if is_part_number && !potential_part_number.is_empty() {
                    engine_schematic_total += potential_part_number.parse::<u32>().unwrap();
                }

                potential_part_number.clear();
                is_part_number = false;
            }
        }
    }

    Some(engine_schematic_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut potential_part_number: String = String::new();
    let mut found_gears: Vec<(usize, usize)> = Vec::new();

    for (i, line) in map.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                potential_part_number.push(c);

                find_gears(&map, i as i32, j as i32, &mut found_gears);
            }

            if !c.is_ascii_digit() || j == line.len() - 1 {
                if !found_gears.is_empty() && !potential_part_number.is_empty() {
                    let num = potential_part_number.parse::<u32>().unwrap();
                    for gear in found_gears.iter().unique() {
                        gears.entry(*gear).or_default().push(num);
                    }
                }

                potential_part_number.clear();
                found_gears.clear();
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
