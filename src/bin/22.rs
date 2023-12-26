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

fn bricks_intersect(brick1: &Brick, brick2: &Brick) -> bool {
    (brick1.end.z >= brick2.start.z && brick1.start.z <= brick2.start.z)
        && brick1.start.x <= brick2.end.x
        && brick2.start.x <= brick1.end.x
        && brick1.start.y <= brick2.end.y
        && brick2.start.y <= brick1.end.y
}

fn try_settle(bricks: &[Brick]) -> (Vec<Brick>, u32) {
    let mut settled_bricks: Vec<Brick> = Vec::new();

    let mut counter = 0;

    for brick in bricks.iter() {
        let mut new_position = brick.clone();

        if settled_bricks.is_empty() && new_position.start.z != 1 {
            new_position.start.z = 1;
            new_position.end.z = 1 + (brick.end.z - brick.start.z);
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
                .any(|b| bricks_intersect(b, &test_position))
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

    let mut disintegratable = 0;

    for i in 0..settled_bricks.len() {
        let mut test_bricks = Vec::new();

        for j in 0..settled_bricks.len() {
            if i == j {
                continue;
            }

            test_bricks.push(settled_bricks[j].clone());
        }

        let (_, counter) = try_settle(&test_bricks);

        if counter == 0 {
            disintegratable += 1;
        }
    }

    Some(disintegratable)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bricks = parse(input);

    let (settled_bricks, _) = try_settle(&bricks);

    let mut total_counter = 0;

    for i in 0..settled_bricks.len() {
        let mut test_bricks = Vec::new();

        for j in 0..settled_bricks.len() {
            if i == j {
                continue;
            }

            test_bricks.push(settled_bricks[j].clone());
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
