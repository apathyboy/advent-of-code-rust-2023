use cached::proc_macro::cached;
use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HotspringCondition {
    Operational,
    Damaged,
    Unknown,
}

fn parse_hotsprings(input: &str) -> Vec<HotspringCondition> {
    input
        .chars()
        .map(|c| match c {
            '.' => HotspringCondition::Operational,
            '#' => HotspringCondition::Damaged,
            '?' => HotspringCondition::Unknown,
            _ => panic!("Invalid condition: {}", c),
        })
        .collect()
}

fn parse_contiguous_damaged(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[cached]
fn find_arrangements(
    hotsprings: Vec<HotspringCondition>,
    within_run: Option<usize>,
    contiguous_damaged: Vec<usize>,
) -> usize {
    if hotsprings.is_empty() {
        if (within_run.is_none() && contiguous_damaged.is_empty())
            || (contiguous_damaged.len() == 1
                && within_run.is_some()
                && within_run.unwrap() == contiguous_damaged[0])
        {
            return 1;
        } else {
            return 0;
        }
    }

    let mut possible_more = 0;
    for hotspring in hotsprings.iter() {
        if *hotspring == HotspringCondition::Damaged || *hotspring == HotspringCondition::Unknown {
            possible_more += 1;
        }
    }

    if (within_run.is_some()
        && possible_more + within_run.unwrap() < contiguous_damaged.iter().sum())
        || (within_run.is_none() && possible_more < contiguous_damaged.iter().sum())
        || (within_run.is_some() && contiguous_damaged.is_empty())
        || (hotsprings[0] == HotspringCondition::Operational
            && within_run.is_some()
            && within_run.unwrap() != contiguous_damaged[0])
    {
        return 0;
    }

    let mut arrangements = 0;

    if (hotsprings[0] == HotspringCondition::Operational && within_run.is_some())
        || (hotsprings[0] == HotspringCondition::Unknown
            && within_run.is_some()
            && within_run.unwrap() == contiguous_damaged[0])
    {
        arrangements += find_arrangements(
            hotsprings.iter().skip(1).copied().collect(),
            None,
            contiguous_damaged.iter().skip(1).copied().collect(),
        );
    }

    if (hotsprings[0] == HotspringCondition::Damaged
        || hotsprings[0] == HotspringCondition::Unknown)
        && within_run.is_some()
    {
        arrangements += find_arrangements(
            hotsprings.iter().skip(1).copied().collect(),
            Some(within_run.unwrap() + 1),
            contiguous_damaged.clone(),
        );
    }

    if (hotsprings[0] == HotspringCondition::Damaged
        || hotsprings[0] == HotspringCondition::Unknown)
        && within_run.is_none()
    {
        arrangements += find_arrangements(
            hotsprings.iter().skip(1).copied().collect(),
            Some(1),
            contiguous_damaged.clone(),
        );
    }

    if (hotsprings[0] == HotspringCondition::Operational
        || hotsprings[0] == HotspringCondition::Unknown)
        && within_run.is_none()
    {
        arrangements += find_arrangements(
            hotsprings.iter().skip(1).copied().collect(),
            None,
            contiguous_damaged.clone(),
        );
    }

    arrangements
}

pub fn part_one(input: &str) -> Option<usize> {
    let arrangements = input
        .lines()
        .map(|l| {
            let (hotsprings, contiguous_damaged) = l.split(' ').collect_tuple().unwrap();
            let hotsprings = parse_hotsprings(hotsprings);
            let contiguous_damaged = parse_contiguous_damaged(contiguous_damaged);

            find_arrangements(hotsprings, None, contiguous_damaged)
        })
        .sum();

    Some(arrangements)
}

pub fn part_two(input: &str) -> Option<usize> {
    let arrangements = input
        .lines()
        .map(|l| {
            let (hotsprings, contiguous_damaged) = l.split(' ').collect_tuple().unwrap();
            let hotsprings = parse_hotsprings(hotsprings);
            let contiguous_damaged = parse_contiguous_damaged(contiguous_damaged);

            let mut hotsprings = vec![hotsprings; 5]
                .into_iter()
                .flat_map(|inner_vec| {
                    let mut with_sep = inner_vec;
                    with_sep.push(HotspringCondition::Unknown);
                    with_sep
                })
                .collect::<Vec<_>>();
            hotsprings.pop();

            let contiguous_damaged = vec![contiguous_damaged; 5]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            find_arrangements(hotsprings, None, contiguous_damaged)
        })
        .sum();

    Some(arrangements)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_arrangements() {
        let hotsprings = parse_hotsprings("???.###");
        let contiguous_damaged = parse_contiguous_damaged("1,1,3");

        assert_eq!(find_arrangements(hotsprings, None, contiguous_damaged), 1);

        let hotsprings = parse_hotsprings(
            ".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.",
        );
        let contiguous_damaged = parse_contiguous_damaged("1,1,3,1,1,3,1,1,3,1,1,3,1,1,3");

        assert_eq!(
            find_arrangements(hotsprings, None, contiguous_damaged),
            16384
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
