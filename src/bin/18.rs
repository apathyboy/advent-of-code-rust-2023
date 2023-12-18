use advent_of_code::Point2D;

advent_of_code::solution!(18);

fn parse_color(color: &str) -> (isize, isize) {
    // remove the # and the parens
    // take the first 5 characters and convert from hex to decimal
    // take the last character and convert from hex to decimal
    let color = &color[2..color.len() - 1];

    let (dist, dir) = color.split_at(5);
    let dir = isize::from_str_radix(dir, 16).unwrap();
    let dist = isize::from_str_radix(dist, 16).unwrap();

    (dir, dist)
}

fn polygon_area(points: &[Point2D]) -> isize {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        sum1 += p1.x * p2.y;
        sum2 += p1.y * p2.x;
    }

    (sum1 - sum2).abs() / 2
}

pub fn part_one(input: &str) -> Option<u32> {
    // map of y to min_x and max_x
    let mut outline: Vec<Point2D> = Vec::new();
    let mut outline_length = 0;

    let mut pos = Point2D::new(0, 0);

    for line in input.lines() {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let distance = parts.next().unwrap().parse::<isize>().unwrap();

        outline_length += distance;

        match direction {
            "R" => {
                pos.x += distance;
            }
            "L" => {
                pos.x -= distance;
            }
            "U" => {
                pos.y -= distance;
            }
            "D" => {
                pos.y += distance;
            }
            _ => panic!("Unknown direction {}", direction),
        }
        outline.push(pos);
    }

    Some((((outline_length / 2) + 1) + polygon_area(&outline)) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    // map of y to min_x and max_x
    let mut outline: Vec<Point2D> = Vec::new();
    let mut outline_length = 0;

    let mut pos = Point2D::new(0, 0);

    for line in input.lines() {
        let mut parts = line.split(' ');
        let color = parts.nth(2).unwrap();

        let (direction, distance) = parse_color(color);

        outline_length += distance;

        match direction {
            0 => {
                pos.x += distance;
            }
            2 => {
                pos.x -= distance;
            }
            3 => {
                pos.y -= distance;
            }
            1 => {
                pos.y += distance;
            }
            _ => panic!("Unknown direction {}", direction),
        }
        outline.push(pos);
    }

    Some((((outline_length / 2) + 1) + polygon_area(&outline)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let (dir, dist) = parse_color("(#70c710)");
        assert_eq!(dir, 0);
        assert_eq!(dist, 461937);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
