use advent_of_code::Point2D;
use itertools::Itertools;
use pathfinding::prelude::Matrix;
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

fn find_reachable_points(map: HashMap<Point2D, char>, max_steps: usize) -> usize {
    let start = map
        .iter()
        .find_map(|(pos, &c)| if c == 'S' { Some(pos) } else { None })
        .unwrap();

    let width = map.keys().map(|pos| pos.x).max().unwrap();
    let height = map.keys().map(|pos| pos.y).max().unwrap();

    dbg!(width, height);

    let mut queue = vec![*start];
    let mut next_queue = vec![];

    let mut steps = 0;

    while steps < max_steps {
        while let Some(pos) = queue.pop() {
            let mut next = pos;
            next.x = (next.x + 1).rem_euclid(width);
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next) {
                next_queue.push(next);
            }

            let mut next = pos;
            next.x = (next.x - 1).rem_euclid(width);
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next) {
                next_queue.push(next);
            }

            let mut next = pos;
            next.y = (next.y + 1).rem_euclid(height);
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next) {
                next_queue.push(next);
            }

            let mut next = pos;
            next.y = (next.y - 1).rem_euclid(height);
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next) {
                next_queue.push(next);
            }
        }

        (queue, next_queue) = (next_queue, queue);
        next_queue.clear();

        steps += 1;
    }

    queue.iter().unique().count()
}

fn part(input: &str, goal: usize) -> usize {
    let grid = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();
    let (sr, sc) = grid
        .items()
        .find_map(|(pos, b)| (*b == b'S').then_some(pos))
        .unwrap();
    let (g, mut ys, mut reachable) = (grid.rows, vec![], HashSet::new());
    reachable.insert((sr as isize, sc as isize));
    for count in 1..=goal {
        for (r, c) in reachable.drain().collect::<Vec<_>>() {
            reachable.extend(
                [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)]
                    .iter()
                    .filter(|&&(nr, nc)| grid[grid.constrain((nr, nc))] != b'#'),
            );
        }
        if count % g == g / 2 {
            ys.push(reachable.len());
            if let &[y0, y1, y2] = &ys[..] {
                let x = goal / g;
                return (x * x * (y0 + y2 - 2 * y1) + x * (4 * y1 - 3 * y0 - y2) + 2 * y0) / 2;
            }
        }
    }
    reachable.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);

    Some(find_reachable_points(map, 64))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(part(input, 26501365))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = parse_map(&advent_of_code::template::read_file("examples", DAY));
        let result = find_reachable_points(map, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_part_two() {
        let result = part(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 50);
    }
}
