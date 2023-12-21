use advent_of_code::Point2D;
use itertools::Itertools;
use std::collections::HashMap;

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

    let mut visited = vec![*start];
    let mut queue = vec![*start];
    let mut next_queue = vec![];

    let mut steps = 0;

    while steps < max_steps {
        while let Some(pos) = queue.pop() {
            let mut next = pos;
            next.x += 1;
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next)
            /* && !visited.contains(&next) */
            {
                next_queue.push(next);
                visited.push(next);
            }

            let mut next = pos;
            next.x -= 1;
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next)
            /* && !visited.contains(&next) */
            {
                next_queue.push(next);
                visited.push(next);
            }

            let mut next = pos;
            next.y += 1;
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next)
            /* && !visited.contains(&next) */
            {
                next_queue.push(next);
                visited.push(next);
            }

            let mut next = pos;
            next.y -= 1;
            if map.contains_key(&next) && map[&next] != '#' && !next_queue.contains(&next)
            /* && !visited.contains(&next) */
            {
                next_queue.push(next);
                visited.push(next);
            }
        }

        queue = next_queue;
        next_queue = vec![];

        steps += 1;
    }

    dbg!(queue.iter().unique().count());

    queue.iter().unique().count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);

    Some(find_reachable_points(map, 64))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
