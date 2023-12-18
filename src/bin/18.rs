use advent_of_code::Point2D;

advent_of_code::solution!(18);

fn parse_part_one(line: &str) -> (Point2D, usize) {
    let mut parts = line.split(' ');
    let direction = parts.next().unwrap();
    let distance = parts.next().unwrap().parse::<isize>().unwrap();
    let offset = match direction {
        "R" => Point2D::new(distance, 0),
        "L" => Point2D::new(-distance, 0),
        "U" => Point2D::new(0, -distance),
        "D" => Point2D::new(0, distance),
        _ => panic!("Unknown direction {}", direction),
    };

    (offset, distance as usize)
}

fn parse_part_two(line: &str) -> (Point2D, usize) {
    let color = line.split(' ').nth(2).unwrap();
    let (direction, distance) = parse_color(color);
    let offset = match direction {
        0 => Point2D::new(distance, 0),
        2 => Point2D::new(-distance, 0),
        3 => Point2D::new(0, -distance),
        1 => Point2D::new(0, distance),
        _ => panic!("Unknown direction {}", direction),
    };

    (offset, distance as usize)
}

fn parse_color(color: &str) -> (isize, isize) {
    let color = &color[2..color.len() - 1];

    let (dist, dir) = color.split_at(5);
    let dir = isize::from_str_radix(dir, 16).unwrap();
    let dist = isize::from_str_radix(dist, 16).unwrap();

    (dir, dist)
}

fn polygon_area(points: &[Point2D], perimeter_length: usize) -> usize {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        sum1 += p1.x * p2.y;
        sum2 += p1.y * p2.x;
    }

    ((perimeter_length / 2) + 1) + ((sum1 - sum2).unsigned_abs() / 2)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (perimeter, perimeter_lengths): (Vec<_>, Vec<_>) = input
        .lines()
        .map(parse_part_one)
        .scan(
            (Point2D::new(0, 0), 0),
            |(pos, perimeter_length), (offset, distance)| {
                *pos = pos.add(&offset);
                *perimeter_length += distance;

                Some((*pos, *perimeter_length))
            },
        )
        .unzip();

    Some(polygon_area(
        &perimeter,
        perimeter_lengths[perimeter_lengths.len() - 1],
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (perimeter, perimeter_lengths): (Vec<_>, Vec<_>) = input
        .lines()
        .map(parse_part_two)
        .scan(
            (Point2D::new(0, 0), 0),
            |(pos, perimeter_length), (offset, distance)| {
                *pos = pos.add(&offset);
                *perimeter_length += distance;

                Some((*pos, *perimeter_length))
            },
        )
        .unzip();

    Some(polygon_area(
        &perimeter,
        perimeter_lengths[perimeter_lengths.len() - 1],
    ))
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
