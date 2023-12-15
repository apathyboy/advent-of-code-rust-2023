use std::{collections::VecDeque, fmt};

advent_of_code::solution!(15);

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

// custom Lens printing
impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

#[derive(Debug)]
enum Operation {
    Remove,
    AddOrUpdate,
}

#[derive(Debug)]
struct Step {
    lens: String,
    box_id: usize,
    operation: Operation,
    focal_length: Option<u32>,
}

fn hash(input: &str) -> Option<u32> {
    let mut current_value = 0;

    for c in input.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    Some(current_value)
}

fn parse_step(input: &str) -> Step {
    let parts = input.split_inclusive(&['-', '=']).collect::<Vec<&str>>();
    let label = &parts[0][..parts[0].len() - 1];
    let operation = match parts[0].chars().last().unwrap() {
        '=' => Operation::AddOrUpdate,
        '-' => Operation::Remove,
        _ => panic!("Unknown operation"),
    };
    let focal_length = if parts.len() == 2 {
        Some(parts[1].parse::<u32>().unwrap())
    } else {
        None
    };

    Step {
        lens: label.to_string(),
        box_id: hash(label).unwrap() as usize,
        operation,
        focal_length,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.trim().split(',').map(hash).sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<VecDeque<Lens>> = Vec::new();

    for _ in 0..256 {
        boxes.push(VecDeque::new());
    }

    for step in input.trim().split(',').map(parse_step) {
        match step.operation {
            Operation::AddOrUpdate => {
                if let Some(index) = boxes[step.box_id].iter().position(|l| l.label == step.lens) {
                    boxes[step.box_id][index].focal_length = step.focal_length.unwrap();
                } else {
                    boxes[step.box_id].push_back(Lens {
                        label: step.lens,
                        focal_length: step.focal_length.unwrap(),
                    });
                }
            }
            Operation::Remove => {
                if let Some(index) = boxes[step.box_id].iter().position(|l| l.label == step.lens) {
                    boxes[step.box_id].remove(index);
                }
            }
        }
    }

    let mut total = 0;

    for (i, box_contents) in boxes.iter().enumerate() {
        if box_contents.is_empty() {
            continue;
        }

        for (j, lens) in box_contents.iter().enumerate() {
            total += (i as u32 + 1) * (j as u32 + 1) * lens.focal_length;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
