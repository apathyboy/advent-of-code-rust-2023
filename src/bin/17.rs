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
    let mut distances: HashMap<(Point2D, Point2D), u32> = HashMap::new();
    let mut visit: VecDeque<(Point2D, Point2D, u32)> = VecDeque::new();
    visit.push_back((Point2D::new(0, 0), Point2D::new(1, 0), 1));

    let lines = input.lines().collect::<Vec<&str>>();
    let width = lines[0].len();
    let height = lines.len();

    // implement dijkstra's algorithm
    // 1. set the distance to the starting node to 0
    // 2. set the distance to all other nodes to infinity
    for (point, _) in edge_weights.iter() {
        if *point == Point2D::new(0, 0) {
            distances.insert((*point, Point2D::new(1, 0)), 0);
            distances.insert((*point, Point2D::new(-1, 0)), std::u32::MAX);
            distances.insert((*point, Point2D::new(0, -1)), std::u32::MAX);
            distances.insert((*point, Point2D::new(0, 1)), std::u32::MAX);
        } else {
            distances.insert((*point, Point2D::new(-1, 0)), std::u32::MAX);
            distances.insert((*point, Point2D::new(1, 0)), std::u32::MAX);
            distances.insert((*point, Point2D::new(0, -1)), std::u32::MAX);
            distances.insert((*point, Point2D::new(0, 1)), std::u32::MAX);
        }
    }

    while !visit.is_empty() {
        // 3. visit the next unvisited node
        // 4. for the current node, consider all of its unvisited neighbors and calculate their tentative distances
        // 5. when we are done considering all of the neighbors of the current node, mark the current node as visited
        // 6. if the destination node has been marked visited or if the smallest tentative distance among the nodes in the unvisited set is infinity, then stop. the algorithm has finished
        // 7. otherwise, select the unvisited node that is marked with the smallest tentative distance, set it as the new "current node", and go back to step 3
        let (cur_pos, cur_dir, counter) = visit.pop_front().unwrap();
        let current_distance = *distances.get(&(cur_pos, cur_dir)).unwrap();

        for step in [
            Point2D::new(-1, 0),
            Point2D::new(1, 0),
            Point2D::new(0, -1),
            Point2D::new(0, 1),
        ] {
            let dir_to_previous = cur_dir.mul_scalar(-1);
            if step == dir_to_previous || (counter >= 3 && step == cur_dir) {
                continue;
            }

            let next_pos = cur_pos.add(&step);
            let next_dir = step;
            let next_counter = if step == cur_dir { counter + 1 } else { 1 };

            if edge_weights.contains_key(&next_pos) {
                let next_distance = current_distance + edge_weights.get(&next_pos).unwrap();

                if next_distance < *distances.get(&(next_pos, next_dir)).unwrap() {
                    distances.insert((next_pos, next_dir), next_distance);
                    visit.push_back((next_pos, next_dir, next_counter));
                }
            }
        }

        //let (next_pos, next_dir) = visit.pop_front().unwrap();
        //let current_distance = distances.get(&(next_pos, next_dir)).unwrap();
    }

    let (_, distance) = distances
        .iter()
        .filter(|((pos, _), _)| *pos == Point2D::new(width as isize - 1, height as isize - 1))
        .min_by_key(|(_, distance)| **distance)
        .unwrap();

    // 4. for the current node, consider all of its unvisited neighbors and calculate their tentative distances
    // 5. when we are done considering all of the neighbors of the current node, mark the current node as visited
    // 6. if the destination node has been marked visited or if the smallest tentative distance among the nodes in the unvisited set is infinity, then stop. the algorithm has finished
    // 7. otherwise, select the unvisited node that is marked with the smallest tentative distance, set it as the new "current node", and go back to step 3

    Some(*distance)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
