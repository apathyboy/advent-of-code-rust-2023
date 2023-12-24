extern crate nalgebra as na;
advent_of_code::solution!(24);

#[derive(Debug)]
struct Hailstone {
    position: na::Vector3<f64>,
    velocity: na::Vector3<f64>,
}

impl Hailstone {
    fn new(position: na::Vector3<f64>, velocity: na::Vector3<f64>) -> Self {
        Self { position, velocity }
    }
}

fn parse_hailstone(line: &str) -> Option<Hailstone> {
    let (pos_str, vel_str) = line.split_once(" @ ").unwrap();

    let position = pos_str
        .split(", ")
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let velocity = vel_str
        .split(", ")
        .map(|s| s.trim().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    Some(Hailstone::new(
        na::Vector3::new(position[0] as f64, position[1] as f64, position[2] as f64),
        na::Vector3::new(velocity[0] as f64, velocity[1] as f64, velocity[2] as f64),
    ))
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter_map(parse_hailstone)
        .collect::<Vec<_>>()
}

fn intersects(
    point_a: &na::Vector3<f64>,
    velocity_a: &na::Vector3<f64>,
    point_b: &na::Vector3<f64>,
    velocity_b: &na::Vector3<f64>,
) -> Option<na::Vector2<f64>> {
    let slope_a = velocity_a.y / velocity_a.x;
    let slope_b = velocity_b.y / velocity_b.x;

    if slope_a == slope_b {
        return None;
    }

    let x =
        (point_b.y - point_a.y + slope_a * point_a.x - slope_b * point_b.x) / (slope_a - slope_b);
    let y = slope_a * (x - point_a.x) + point_a.y;

    if is_future(point_a, velocity_a, x, y) && is_future(point_b, velocity_b, x, y) {
        Some(na::Vector2::new(x, y))
    } else {
        None
    }
}

fn is_future(point: &na::Vector3<f64>, velocity: &na::Vector3<f64>, x: f64, y: f64) -> bool {
    if velocity.x < 0.0 && point.x < x {
        return false;
    }
    if velocity.x > 0.0 && point.x > x {
        return false;
    }
    if velocity.y < 0.0 && point.y < y {
        return false;
    }
    if velocity.y > 0.0 && point.y > y {
        return false;
    }

    true
}

fn count_valid_intersections(
    hailstones: &[Hailstone],
    min_bound: isize,
    max_bound: isize,
) -> usize {
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hailstones[i + 1..]
                .iter()
                .map(move |b| intersects(&a.position, &a.velocity, &b.position, &b.velocity))
        })
        .filter(|i| {
            if let Some(intersection) = i {
                //dbg!(intersection);

                intersection.x >= min_bound as f64
                    && intersection.x <= max_bound as f64
                    && intersection.y >= min_bound as f64
                    && intersection.y <= max_bound as f64
            } else {
                false
            }
        })
        .count()
}

fn cross_matrix(v: na::Vector3<f64>) -> na::Matrix3<f64> {
    na::Matrix3::new(0.0, -v[2], v[1], v[2], 0.0, -v[0], -v[1], v[0], 0.0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let hailstones = parse_input(input);

    Some(count_valid_intersections(
        &hailstones,
        200000000000000,
        400000000000000,
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let hailstones = parse_input(input);

    let mut m = na::Matrix6::<f64>::zeros();
    let mut rhs = na::Vector6::<f64>::zeros();

    let cross_pv0 = hailstones[0].position.cross(&hailstones[0].velocity);
    let cross_pv1 = hailstones[1].position.cross(&hailstones[1].velocity);
    let cross_pv2 = hailstones[2].position.cross(&hailstones[2].velocity);

    rhs.fixed_rows_mut::<3>(0)
        .copy_from(&(cross_pv1 - cross_pv0));
    rhs.fixed_rows_mut::<3>(3)
        .copy_from(&(cross_pv2 - cross_pv0));

    m.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&(cross_matrix(hailstones[0].velocity) - cross_matrix(hailstones[1].velocity)));
    m.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&(cross_matrix(hailstones[0].velocity) - cross_matrix(hailstones[2].velocity)));
    m.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&(-cross_matrix(hailstones[0].position) + cross_matrix(hailstones[1].position)));
    m.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&(-cross_matrix(hailstones[0].position) + cross_matrix(hailstones[2].position)));

    let result = m.try_inverse().expect("Matrix inversion failed") * rhs;

    result
        .iter()
        .map(|&x| Some(x.ceil() as usize))
        .take(3)
        .sum()
    //None Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_intersections() {
        let input = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = count_valid_intersections(&input, 7, 27);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
