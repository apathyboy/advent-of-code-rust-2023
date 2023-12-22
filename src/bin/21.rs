use advent_of_code::Point2D;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

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

    let width = map.keys().map(|pos| pos.x).max().unwrap() + 1;
    let height = map.keys().map(|pos| pos.y).max().unwrap() + 1;

    let mut queue = HashSet::from([*start]);
    let mut next_queue = HashSet::new();

    let mut steps = 0;

    while steps < max_steps {
        for pos in queue.iter() {
            for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let mut next = *pos;
                next.x += dir.0;
                next.y += dir.1;

                let mut check = next;
                check.x = check.x.rem_euclid(width);
                check.y = check.y.rem_euclid(height);

                if map[&check] != '#' {
                    next_queue.insert(next);
                }
            }
        }

        queue.clear();
        swap(&mut queue, &mut next_queue);

        steps += 1;
    }

    queue.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);

    Some(find_reachable_points(&map, 64))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_map(input);

    let res = [65, 65 + 131, 65 + 131 * 2]
        .into_par_iter()
        .map(|n| find_reachable_points(&map, n))
        .collect::<Vec<_>>();

    let n = 202300;

    let a2 = res[2] - 2 * res[1] + res[0];
    let b2 = 4 * res[1] - 3 * res[0] - res[2];
    let c = res[0];

    Some((n * n * a2 + n * b2) / 2 + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_reachable_points() {
        let map = parse_map(&advent_of_code::template::read_file("examples", DAY));
        let result = find_reachable_points(&map, 6);
        assert_eq!(result, 16);

        let result = find_reachable_points(&map, 10);
        assert_eq!(result, 50);

        let result = find_reachable_points(&map, 50);
        assert_eq!(result, 1594);

        let result = find_reachable_points(&map, 100);
        assert_eq!(result, 6536);
    }
}
