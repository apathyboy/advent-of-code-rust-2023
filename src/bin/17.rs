use std::collections::{HashMap, VecDeque};

use advent_of_code::Point2D;

advent_of_code::solution!(17);

fn parse_edge_weights(input: &str) -> HashMap<Point2D, u32> {
    let mut edge_weights: HashMap<Point2D, u32> = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, b)| {
            let point = Point2D::new(x as isize, y as isize);
            let weight = b.to_digit(10).unwrap();

            edge_weights.insert(point, weight);
        });
    });

    edge_weights
}

pub fn part_one(input: &str) -> Option<u32> {
    let edge_weights = parse_edge_weights(input);
    let mut distances: HashMap<(Point2D, Point2D, u32), u32> = HashMap::new();
    let mut visit: VecDeque<(Point2D, Point2D, u32)> = VecDeque::new();
    visit.push_back((Point2D::new(0, 0), Point2D::new(1, 0), 0));

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    distances.insert((Point2D::new(0, 0), Point2D::new(1, 0), 0), 0);

    while !visit.is_empty() {
        let (cur_pos, cur_dir, counter) = visit.pop_front().unwrap();
        let current_distance = *distances
            .get(&(cur_pos, cur_dir, counter))
            .unwrap_or(&u32::MAX);

        for step in [
            Point2D::new(-1, 0),
            Point2D::new(1, 0),
            Point2D::new(0, -1),
            Point2D::new(0, 1),
        ] {
            let dir_to_previous = cur_dir.mul_scalar(-1);
            if step == dir_to_previous || (counter == 3 && step == cur_dir) {
                continue;
            }

            let next_pos = cur_pos.add(&step);
            let next_dir = step;
            let next_counter = if step == cur_dir { counter + 1 } else { 1 };

            if edge_weights.contains_key(&next_pos) {
                let next_distance = current_distance + edge_weights.get(&next_pos).unwrap();

                if next_distance
                    < *distances
                        .get(&(next_pos, next_dir, next_counter))
                        .unwrap_or(&u32::MAX)
                {
                    distances.insert((next_pos, next_dir, next_counter), next_distance);
                    visit.push_back((next_pos, next_dir, next_counter));
                }
            }
        }
    }

    let (_, distance) = distances
        .iter()
        .filter(|((pos, _, _), _)| *pos == Point2D::new(width as isize - 1, height as isize - 1))
        .min_by_key(|(_, distance)| **distance)
        .unwrap();

    Some(*distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let edge_weights = parse_edge_weights(input);
    let mut distances: HashMap<(Point2D, Point2D, u32), u32> = HashMap::new();
    let mut visit: VecDeque<(Point2D, Point2D, u32)> = VecDeque::new();
    visit.push_back((Point2D::new(0, 0), Point2D::new(1, 0), 0));

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    distances.insert((Point2D::new(0, 0), Point2D::new(1, 0), 0), 0);

    while !visit.is_empty() {
        let (cur_pos, cur_dir, counter) = visit.pop_front().unwrap();
        let current_distance = *distances
            .get(&(cur_pos, cur_dir, counter))
            .unwrap_or(&u32::MAX);

        for step in [
            Point2D::new(-1, 0),
            Point2D::new(1, 0),
            Point2D::new(0, -1),
            Point2D::new(0, 1),
        ] {
            let dir_to_previous = cur_dir.mul_scalar(-1);
            if step == dir_to_previous
                || (step == cur_dir && counter == 10)
                || (step != cur_dir && counter < 4)
            {
                continue;
            }

            let next_pos = cur_pos.add(&step);
            let next_dir = step;
            let next_counter = if step == cur_dir { counter + 1 } else { 1 };

            // dbg!(next_pos, next_dir, next_counter);

            if edge_weights.contains_key(&next_pos) {
                let next_distance = current_distance + edge_weights.get(&next_pos).unwrap();

                if next_distance
                    < *distances
                        .get(&(next_pos, next_dir, next_counter))
                        .unwrap_or(&u32::MAX)
                {
                    distances.insert((next_pos, next_dir, next_counter), next_distance);
                    visit.push_back((next_pos, next_dir, next_counter));
                }
            }
        }
    }

    let (_, distance) = distances
        .iter()
        .filter(|((pos, _, counter), _)| {
            *pos == Point2D::new(width as isize - 1, height as isize - 1) && *counter >= 4
        })
        .min_by_key(|(_, distance)| **distance)
        .unwrap();

    Some(*distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
