use advent_of_code::Point3D;
use itertools::Itertools;

advent_of_code::solution!(22);

#[derive(Debug, Clone, PartialEq)]
struct Brick {
    start: Point3D,
    end: Point3D,
}

impl Brick {
    fn new(start: Point3D, end: Point3D) -> Self {
        Self { start, end }
    }
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let bounds: Vec<_> = line
                .split('~')
                .map(|part| {
                    let parts = part
                        .split(',')
                        .map(|item| item.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>();

                    Point3D::new(parts[0], parts[1], parts[2])
                })
                .collect();
            if bounds[0].z > bounds[1].z {
                Brick::new(bounds[1], bounds[0])
            } else {
                Brick::new(bounds[0], bounds[1])
            }
        })
        .sorted_by(|a, b| a.start.z.cmp(&b.start.z))
        .collect()
}

fn do_segments_intersect(segment1: &Brick, segment2: &Brick) -> bool {
    let p1 = &segment1.start;
    let q1 = &segment1.end;
    let p2 = &segment2.start;
    let q2 = &segment2.end;

    // Check if the segments are collinear and overlap on the x-axis or z-axis
    if p1.x == q1.x && p2.x == q2.x && p1.x == p2.x {
        let y1_min = p1.y.min(q1.y);
        let y1_max = p1.y.max(q1.y);
        let y2_min = p2.y.min(q2.y);
        let y2_max = p2.y.max(q2.y);

        let z1_min = p1.z.min(q1.z);
        let z1_max = p1.z.max(q1.z);
        let z2_min = p2.z.min(q2.z);
        let z2_max = p2.z.max(q2.z);

        return (y1_min <= y2_max && y2_min <= y1_max) && (z1_min <= z2_max && z2_min <= z1_max);
    }

    // Check if the segments are collinear and overlap on the y-axis or z-axis
    if p1.y == q1.y && p2.y == q2.y && p1.y == p2.y {
        let x1_min = p1.x.min(q1.x);
        let x1_max = p1.x.max(q1.x);
        let x2_min = p2.x.min(q2.x);
        let x2_max = p2.x.max(q2.x);

        let z1_min = p1.z.min(q1.z);
        let z1_max = p1.z.max(q1.z);
        let z2_min = p2.z.min(q2.z);
        let z2_max = p2.z.max(q2.z);

        return (x1_min <= x2_max && x2_min <= x1_max) && (z1_min <= z2_max && z2_min <= z1_max);
    }

    // Check if the segments are collinear and overlap on the x-axis or y-axis
    if p1.z == q1.z && p2.z == q2.z && p1.z == p2.z {
        let x1_min = p1.x.min(q1.x);
        let x1_max = p1.x.max(q1.x);
        let x2_min = p2.x.min(q2.x);
        let x2_max = p2.x.max(q2.x);

        let y1_min = p1.y.min(q1.y);
        let y1_max = p1.y.max(q1.y);
        let y2_min = p2.y.min(q2.y);
        let y2_max = p2.y.max(q2.y);

        return (x1_min <= x2_max && x2_min <= x1_max) && (y1_min <= y2_max && y2_min <= y1_max);
    }

    false
}

fn try_settle(bricks: &[Brick]) -> (Vec<Brick>, u32) {
    let mut settled_bricks: Vec<Brick> = Vec::new();

    let mut counter = 0;

    for brick in bricks.iter() {
        let mut new_position = brick.clone();

        if settled_bricks.is_empty() && new_position.start.z != 1 {
            new_position.start.z = 1;
            new_position.end.z = 1;
            settled_bricks.push(new_position);
            counter += 1;
            continue;
        }

        let mut fell = false;

        loop {
            let mut test_position = new_position.clone();
            test_position.start.z -= 1;
            test_position.end.z -= 1;

            if test_position.start.z < 1 {
                break;
            }

            if settled_bricks
                .iter()
                .filter(|b| b.start.z == test_position.start.z)
                .any(|b| do_segments_intersect(b, &test_position))
            {
                break;
            }

            fell = true;

            new_position = test_position;
        }

        if fell {
            counter += 1;
        }

        settled_bricks.push(new_position);
    }

    (settled_bricks, counter)
}

pub fn part_one(input: &str) -> Option<u32> {
    let bricks = parse(input);

    let (settled_bricks, _) = try_settle(&bricks);

    let mut disintegratable = Vec::new();

    for i in 0..settled_bricks.len() {
        let mut test_bricks = Vec::new();

        let to_remove = settled_bricks[i].clone();

        for settled_brick in settled_bricks.iter() {
            if settled_brick == &to_remove {
                continue;
            }

            test_bricks.push(settled_brick.clone());
        }

        let (_, counter) = try_settle(&test_bricks);

        if counter == 0 {
            disintegratable.push(to_remove);
        }
    }

    Some(disintegratable.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bricks = parse(input);

    let (settled_bricks, _) = try_settle(&bricks);

    let mut total_counter = 0;

    for i in 0..settled_bricks.len() {
        let mut test_bricks = Vec::new();

        let to_remove = settled_bricks[i].clone();

        for settled_brick in settled_bricks.iter() {
            if settled_brick == &to_remove {
                continue;
            }

            test_bricks.push(settled_brick.clone());
        }

        let (_, counter) = try_settle(&test_bricks);

        total_counter += counter;
    }

    Some(total_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
