use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    VerticalPipe,
    HorizontalPipe,
    NorthEastPipe,
    NorthWestPipe,
    SouthWestPipe,
    SouthEastPipe,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            '.' => Tile::Ground,
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::NorthEastPipe,
            'J' => Tile::NorthWestPipe,
            '7' => Tile::SouthWestPipe,
            'F' => Tile::SouthEastPipe,
            _ => panic!("Unknown tile: {}", c),
        }
    }
}

fn find_next_position(
    map: &[Vec<Tile>],
    visited: &[(usize, usize)],
    x: usize,
    y: usize,
) -> Option<(usize, usize)> {
    let directions: Vec<(i32, i32)> = vec![
        (0, -1), // North
        (1, 0),  // East
        (0, 1),  // South
        (-1, 0), // West
    ];

    let current_tile = &map[y][x];

    //println!("Current tile: {:?}", current_tile);

    for (dx, dy) in directions {
        if (x == 0 && dx == -1)
            || (y == 0 && dy == -1)
            || (x == map[0].len() - 1 && dx == 1)
            || (y == map.len() - 1 && dy == 1)
        {
            continue;
        }

        let (x, y) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

        if visited.contains(&(x, y)) {
            continue;
        }

        let tile = &map[y][x];

        if dy == -1
            && (*current_tile == Tile::VerticalPipe
                || *current_tile == Tile::NorthEastPipe
                || *current_tile == Tile::NorthWestPipe
                || *current_tile == Tile::Start)
        {
            match tile {
                Tile::VerticalPipe | Tile::SouthEastPipe | Tile::SouthWestPipe => {
                    // println!("Found pipe at ({}, {}) {:?}", x, y, tile);
                    return Some((x, y));
                }
                _ => {}
            }
        } else if dx == 1
            && (*current_tile == Tile::HorizontalPipe
                || *current_tile == Tile::NorthEastPipe
                || *current_tile == Tile::SouthEastPipe
                || *current_tile == Tile::Start)
        {
            match tile {
                Tile::HorizontalPipe | Tile::NorthWestPipe | Tile::SouthWestPipe => {
                    // println!("Found pipe at ({}, {}) {:?}", x, y, tile);
                    return Some((x, y));
                }
                _ => {}
            }
        } else if dy == 1
            && (*current_tile == Tile::VerticalPipe
                || *current_tile == Tile::SouthEastPipe
                || *current_tile == Tile::SouthWestPipe
                || *current_tile == Tile::Start)
        {
            match tile {
                Tile::VerticalPipe | Tile::NorthEastPipe | Tile::NorthWestPipe => {
                    // println!("Found pipe at ({}, {}) {:?}", x, y, tile);
                    return Some((x, y));
                }
                _ => {}
            }
        } else if dx == -1
            && (*current_tile == Tile::HorizontalPipe
                || *current_tile == Tile::NorthWestPipe
                || *current_tile == Tile::SouthWestPipe
                || *current_tile == Tile::Start)
        {
            match tile {
                Tile::HorizontalPipe | Tile::NorthEastPipe | Tile::SouthEastPipe => {
                    // println!("Found pipe at ({}, {}) {:?}", x, y, tile);
                    return Some((x, y));
                }
                _ => {}
            }
        }
    }

    None
}

fn find_pipe_loop(map: &[Vec<Tile>], start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
    let mut visited: Vec<(usize, usize)> = Vec::new();
    visited.push((start_x, start_y));

    let mut current_position = (start_x, start_y);

    while let Some((x, y)) =
        find_next_position(map, &visited, current_position.0, current_position.1)
    {
        visited.push((x, y));
        current_position = (x, y);
    }

    visited
}
fn find_start_tile_type(
    pipe_section: (usize, usize),
    prev_pipe_section: (usize, usize),
    next_pipe_section: (usize, usize),
) -> Tile {
    let (x, y) = pipe_section;
    let (prev_x, prev_y) = prev_pipe_section;
    let (next_x, next_y) = next_pipe_section;

    if (x == prev_x && y == prev_y - 1 && x == next_x && y == next_y + 1)
        || (x == prev_x && y == prev_y + 1 && x == next_x && y == next_y - 1)
    {
        Tile::VerticalPipe
    } else if (x == prev_x - 1 && y == prev_y && x == next_x + 1 && y == next_y)
        || (x == prev_x + 1 && y == prev_y && x == next_x - 1 && y == next_y)
    {
        Tile::HorizontalPipe
    } else if (x == prev_x + 1 && y == prev_y && x == next_x && y == next_y - 1)
        || (x == prev_x && y == prev_y - 1 && x == next_x + 1 && y == next_y)
    {
        Tile::SouthWestPipe
    } else if (x == prev_x && y == prev_y - 1 && x == next_x - 1 && y == next_y)
        || (x == prev_x - 1 && y == prev_y && x == next_x && y == next_y - 1)
    {
        Tile::SouthEastPipe
    } else if (x == prev_x && y == prev_y + 1 && x == next_x - 1 && y == next_y)
        || (x == prev_x - 1 && y == prev_y && x == next_x && y == next_y + 1)
    {
        Tile::NorthEastPipe
    } else if (x == prev_x + 1 && y == prev_y && x == next_x && y == next_y + 1)
        || (x == prev_x && y == prev_y + 1 && x == next_x + 1 && y == next_y)
    {
        Tile::NorthWestPipe
    } else {
        panic!(
            "Unknown pipe type: ({}, {}) ({}, {}) ({}, {})",
            x, y, prev_x, prev_y, next_x, next_y
        );
    }
}

fn parse(input: &str) -> (Vec<Vec<Tile>>, Vec<(usize, usize)>) {
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let tile = Tile::from(c);

                    if tile == Tile::Start {
                        start_x = x;
                        start_y = y;
                    }

                    tile
                })
                .collect()
        })
        .collect();

    let pipe_loop = find_pipe_loop(&map, start_x, start_y);

    (map, pipe_loop)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, pipe_loop) = parse(input);
    Some(pipe_loop.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, pipe_loop) = parse(input);

    let start_tile =
        find_start_tile_type(pipe_loop[0], pipe_loop[1], pipe_loop[pipe_loop.len() - 1]);

    let map: Vec<Vec<Tile>> = map
        .into_par_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, tile)| match tile {
                    Tile::Start => start_tile,
                    pipe if pipe_loop.contains(&(x, y)) => pipe,
                    _ => Tile::Ground,
                })
                .collect()
        })
        .collect();

    let mut inside = false;
    let tile_count = map
        .into_iter()
        .flatten()
        .filter(|tile| match tile {
            Tile::Ground => inside,
            Tile::VerticalPipe | Tile::NorthWestPipe | Tile::NorthEastPipe => {
                inside = !inside;
                false
            }
            _ => false,
        })
        .count();

    Some(tile_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(10));
    }
}
