advent_of_code::solution!(14);

fn tilt_north(map: &mut [Vec<char>]) {
    for x in 0..map[0].len() {
        let mut y = 0;
        while y < map.len() {
            let y_start = y;

            while y < map.len() && map[y][x] != '#' {
                y += 1;
            }

            for y in y_start..y {
                if map[y][x] == 'O' {
                    for check_y in (y_start..y).rev() {
                        if map[check_y][x] == 'O' {
                            break;
                        }

                        map[check_y + 1][x] = '.';
                        map[check_y][x] = 'O';
                    }
                }
            }

            y += 1;
        }
    }
}

fn tilt_south(map: &mut [Vec<char>]) {
    for x in 0..map[0].len() {
        let mut y = map.len() - 1;

        while y > 0 {
            let y_start = y;

            while y > 0 && map[y][x] != '#' {
                y -= 1;
            }

            for y in (y..y_start).rev() {
                if map[y][x] == 'O' {
                    for check_y in y + 1..=y_start {
                        if map[check_y][x] == 'O' {
                            break;
                        }

                        map[check_y - 1][x] = '.';
                        map[check_y][x] = 'O';
                    }
                }
            }

            match y.checked_sub(1) {
                Some(val) => y = val,
                None => break,
            };
        }
    }
}

fn tilt_west(map: &mut [Vec<char>]) {
    for y in 0..map.len() {
        let mut x = 0;
        while x < map[0].len() {
            let x_start = x;

            while x < map[0].len() && map[y][x] != '#' {
                x += 1;
            }

            for x in x_start..x {
                if map[y][x] == 'O' {
                    for check_x in (x_start..x).rev() {
                        if map[y][check_x] == 'O' {
                            break;
                        }

                        map[y][check_x + 1] = '.';
                        map[y][check_x] = 'O';
                    }
                }
            }

            x += 1;
        }
    }
}

fn tilt_east(map: &mut [Vec<char>]) {
    for y in 0..map.len() {
        let mut x = map[0].len() - 1;

        while x > 0 {
            let x_start = x;

            while x > 0 && map[y][x] != '#' {
                x -= 1;
            }

            for x in (x..x_start).rev() {
                if map[y][x] == 'O' {
                    for check_x in x + 1..=x_start {
                        if map[y][check_x] == 'O' {
                            break;
                        }

                        map[y][check_x - 1] = '.';
                        map[y][check_x] = 'O';
                    }
                }
            }

            match x.checked_sub(1) {
                Some(val) => x = val,
                None => break,
            };
        }
    }
}

fn cycle(map: &mut [Vec<char>]) {
    tilt_north(map);
    tilt_west(map);
    tilt_south(map);
    tilt_east(map);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    tilt_north(&mut map);

    let total_load = map
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() as u32 * (i as u32 + 1))
        .sum();

    Some(total_load)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for _ in 0..1000_i32 {
        cycle(&mut map);
    }

    let total_load = map
        .iter()
        .rev()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() as u32 * (i as u32 + 1))
        .sum();

    Some(total_load)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
