use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
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

// examine the tile to the left of the test position to see if it's part of the pipe loop and if not, continue exploring in all directions until we find a pipe loop tile or an edge tile
// if an edge tile is found, then we know that the pipe loop is on the outer edge of the map
fn explore_lefthand(
    map: &HashMap<(i32, i32), Tile>,
    pipe_loop: &Vec<(i32, i32)>,
    pipe_section: (i32, i32),
    check_direction: (i32, i32),
) -> (Vec<(i32, i32)>, bool) {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut is_inner: bool = true;

    let mut to_check: VecDeque<(i32, i32)> = VecDeque::new();

    to_check.push_back((
        pipe_section.0 + check_direction.0,
        pipe_section.1 + check_direction.1,
    ));

    while !to_check.is_empty() {
        let (x, y) = to_check.pop_front().unwrap();

        if visited.contains(&(x, y)) || pipe_loop.contains(&(x, y)) {
            continue;
        }

        if !map.contains_key(&(x, y)) {
            is_inner = false;
            continue;
        }

        visited.push((x, y));

        to_check.push_back((x, y - 1));
        to_check.push_back((x + 1, y));
        to_check.push_back((x, y + 1));
        to_check.push_back((x - 1, y));
    }

    (visited, is_inner)
}

fn find_next_position(
    map: &HashMap<(i32, i32), Tile>,
    visited: &Vec<(i32, i32)>,
    x: i32,
    y: i32,
) -> Option<(i32, i32)> {
    let directions = vec![
        (0, -1), // North
        (1, 0),  // East
        (0, 1),  // South
        (-1, 0), // West
    ];

    let current_tile = map.get(&(x, y)).unwrap();

    //println!("Current tile: {:?}", current_tile);

    for (dx, dy) in directions {
        let (x, y) = (x + dx, y + dy);

        if visited.contains(&(x, y)) {
            continue;
        }

        let tile = map.get(&(x, y));

        if !tile.is_some() {
            continue;
        }

        let tile = tile.unwrap();

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

fn find_pipe_loop(map: &HashMap<(i32, i32), Tile>, start_x: i32, start_y: i32) -> Vec<(i32, i32)> {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    visited.push((start_x, start_y));

    let mut current_position = (start_x, start_y);

    loop {
        match find_next_position(&map, &visited, current_position.0, current_position.1) {
            Some((x, y)) => {
                visited.push((x, y));
                current_position = (x, y);
            }
            None => {
                //println!("No more pipes found");
                break;
            }
        }
    }

    visited
}

fn parse_map(input: &str) -> (HashMap<(i32, i32), Tile>, Vec<(i32, i32)>) {
    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut start_x = 0;
    let mut start_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Ground,
                'S' => Tile::Start,
                '|' => Tile::VerticalPipe,
                '-' => Tile::HorizontalPipe,
                'L' => Tile::NorthEastPipe,
                'J' => Tile::NorthWestPipe,
                '7' => Tile::SouthWestPipe,
                'F' => Tile::SouthEastPipe,
                _ => panic!("Unknown tile: {}", c),
            };

            if c == 'S' {
                start_x = x as i32;
                start_y = y as i32;
            }

            map.insert((x as i32, y as i32), tile);
        }
    }

    let pipe_loop = find_pipe_loop(&map, start_x, start_y);

    (map, pipe_loop)
}

fn find_start_tile_type(
    pipe_section: (i32, i32),
    prev_pipe_section: (i32, i32),
    next_pipe_section: (i32, i32),
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

pub fn part_one(input: &str) -> Option<u32> {
    let (_, pipe_loop) = parse_map(input);

    Some(pipe_loop.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, pipe_loop) = parse_map(input);

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut is_inner: bool = true;

    println!("pipe_loop: {:?}", pipe_loop);
    let mut prev_tile =
        find_start_tile_type(pipe_loop[0], pipe_loop[1], pipe_loop[pipe_loop.len() - 1]);
    let mut check_direction = (0, -1);

    for (i, (x, y)) in pipe_loop.iter().enumerate() {
        println!("i: {}, x: {}, y: {}", i, *x, *y);
        let pipe_tile = map.get(&(*x, *y)).unwrap();

        println!("prev_tile: {:?}", prev_tile);

        //if i != 0 {
        //    match pipe_tile {
        //        Tile::VerticalPipe {
        //            if check_direction == ()
        //        }
        //        _ => {}
        //    }
        //}

        println!("Checking ({}, {}) {:?}", x, y, check_direction);

        let (found, is_inner_check) = explore_lefthand(&map, &pipe_loop, (*x, *y), check_direction);

        println!("Found: {:?}", found);
        println!("Is inner: {}", is_inner_check);

        visited.extend(found);

        if !is_inner_check {
            is_inner = false;
        }
    }

    visited = visited.iter().unique().cloned().collect();

    println!("is_inner: {}", is_inner);
    println!("map.len(): {}", map.len());
    println!("pipe_loop.len(): {}", pipe_loop.len());
    println!("visited.len(): {}", visited.len());

    if is_inner {
        Some(visited.len() as u32)
    } else {
        Some((map.len() - pipe_loop.len() - visited.iter().unique().count()) as u32)
    }
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
}
