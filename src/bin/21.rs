use advent_of_code::Point2D;
use core::panic;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(21);

fn parse_map(input: &str) -> HashMap<Point2D, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Point2D::new(x as isize, y as isize), c))
        })
        .collect()
}

fn find_reachable_points(map: &HashMap<Point2D, char>, max_steps: usize) -> usize {
    let start = map
        .iter()
        .find_map(|(pos, &c)| if c == 'S' { Some(pos) } else { None })
        .unwrap();

    let width = map.keys().map(|pos| pos.x).max().unwrap();
    let height = map.keys().map(|pos| pos.y).max().unwrap();

    let mut queue = HashSet::from([*start]);
    let mut next_queue = HashSet::new();

    let mut steps = 0;

    while steps < max_steps {
        for pos in queue.iter() {
            for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let mut next = *pos;
                next.x = next.x + dir.0;
                next.y = next.y + dir.1;

                // In order to avoid copies, we can use a checkerboard pattern
                //if steps as isize % 2 != (next.x + next.y).rem_euclid(2) {
                //    continue;
                //}

                let mut check = next;
                check.x = (check.x + (width * max_steps as isize)) % width;
                check.y = (check.y + (height * max_steps as isize)) % height;

                //println!("Checking {:?} {:?}", next, check);
                if map[&check] != '#' {
                    next_queue.insert(next);
                }
            }
        }

        queue.clear();
        (queue, next_queue) = (next_queue, queue);

        steps += 1;

        println!("steps: {} {}", steps, queue.len());
        // panic!();
        //if ((steps as isize - (width / 2)) % 131) == 0 {
        //    dbg!(steps, queue.len());
        //}
    }

    queue.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);

    Some(find_reachable_points(&map, 64))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_map(input);

    let y_0 = find_reachable_points(&map, 65);
    let y_1 = find_reachable_points(&map, 65 + 131);
    let y_2 = find_reachable_points(&map, 65 + 131 * 2);

    let n = 202300;

    let a2 = y_2 - 2 * y_1 + y_0;
    let b2 = 4 * y_1 - 3 * y_0 - y_2;
    let c = y_0;

    println!("{a2}/2 x^2 +{b2}/2 x + {c} = y");
    println!("x=0, y={c}");
    println!("x=1, y={}", (a2 + b2) / 2 + c);
    println!("x=2, y={}", (4 * a2 + 2 * b2) / 2 + c);
    println!("x=202300, y={}", (n * n * a2 + n * b2) / 2 + c);

    Some((n * n * a2 + n * b2) / 2 + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_reachable_points() {
        let map = parse_map(&advent_of_code::template::read_file("examples", DAY));
        //let result = find_reachable_points(&map, 6);
        //assert_eq!(result, 16);

        let result = find_reachable_points(&map, 102);
        assert_eq!(result, 50);
    }
}
