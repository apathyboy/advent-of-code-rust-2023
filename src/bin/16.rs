use advent_of_code::Point2D;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::HashSet; // Add this line to import the `once_cell` crate

advent_of_code::solution!(16);

#[derive(Debug, Copy, Clone)]
struct Beam<'a> {
    direction: &'a Point2D,
    position: Point2D,
}

impl<'a> Beam<'a> {
    fn new(direction: &'a Point2D, position: Point2D) -> Self {
        Self {
            direction,
            position,
        }
    }
}

const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

static DIRECTIONS: Lazy<Vec<Point2D>> = Lazy::new(|| {
    vec![
        Point2D::new(0, -1),
        Point2D::new(0, 1),
        Point2D::new(-1, 0),
        Point2D::new(1, 0),
    ]
});

fn explore(map: &[Vec<char>], starting_beam: &Beam) -> u32 {
    let mut beams: Vec<Beam> = Vec::new();
    let mut energized: HashSet<(Point2D, &Point2D)> = HashSet::new();
    let mut new_beams: Vec<Beam> = Vec::new();

    beams.push(*starting_beam);

    while !beams.is_empty() {
        let mut idx = 0;
        while idx < beams.len() {
            let beam = &mut beams[idx];

            beam.position = beam.position.add(beam.direction);

            if beam.position.x as usize >= map.len()
                || beam.position.y as usize >= map[beam.position.x as usize].len()
                || beam.position.x < 0
                || beam.position.y < 0
                || energized.contains(&(beam.position, beam.direction))
            {
                beams.remove(idx);
                continue;
            }

            energized.insert((beam.position, beam.direction));

            match map[beam.position.y as usize][beam.position.x as usize] {
                '.' => {}
                '/' => match beam.direction {
                    Point2D { x: 1, y: 0 } => {
                        beam.direction = &DIRECTIONS[UP];
                    }
                    Point2D { x: 0, y: 1 } => {
                        beam.direction = &DIRECTIONS[LEFT];
                    }
                    Point2D { x: -1, y: 0 } => {
                        beam.direction = &DIRECTIONS[DOWN];
                    }
                    Point2D { x: 0, y: -1 } => {
                        beam.direction = &DIRECTIONS[RIGHT];
                    }
                    _ => panic!("Invalid beam direction"),
                },
                '\\' => match beam.direction {
                    Point2D { x: 1, y: 0 } => {
                        beam.direction = &DIRECTIONS[DOWN];
                    }
                    Point2D { x: 0, y: 1 } => {
                        beam.direction = &DIRECTIONS[RIGHT];
                    }
                    Point2D { x: -1, y: 0 } => {
                        beam.direction = &DIRECTIONS[UP];
                    }
                    Point2D { x: 0, y: -1 } => {
                        beam.direction = &DIRECTIONS[LEFT];
                    }
                    _ => panic!("Invalid beam direction"),
                },
                '-' => {
                    if beam.direction.y != 0 {
                        beam.direction = &DIRECTIONS[RIGHT];
                        new_beams.push(Beam::new(&DIRECTIONS[LEFT], beam.position));
                    }
                }
                '|' => {
                    if beam.direction.x != 0 {
                        beam.direction = &DIRECTIONS[DOWN];
                        new_beams.push(Beam::new(&DIRECTIONS[UP], beam.position));
                    }
                }
                _ => panic!("Invalid map tile"),
            }

            idx += 1;
        }

        beams.append(&mut new_beams);
        new_beams.clear();
    }

    energized.iter().map(|(p, _)| p).unique().count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let beam = Beam::new(&DIRECTIONS[RIGHT], Point2D::new(-1, 0));

    Some(explore(&map, &beam))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut beams: Vec<Beam> = Vec::new();

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    for x in 0..width {
        beams.push(Beam::new(&DIRECTIONS[DOWN], Point2D::new(x, -1)));
        beams.push(Beam::new(&DIRECTIONS[UP], Point2D::new(x, height)));
    }

    for y in 0..height {
        beams.push(Beam::new(&DIRECTIONS[RIGHT], Point2D::new(-1, y)));
        beams.push(Beam::new(&DIRECTIONS[LEFT], Point2D::new(width, y)));
    }

    beams.into_par_iter().map(|beam| explore(&map, &beam)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
