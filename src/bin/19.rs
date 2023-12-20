use std::{
    cmp::{max, min},
    collections::HashMap,
};

advent_of_code::solution!(19);

#[derive(Debug, PartialEq)]
struct Condition {
    lhs: char,
    operator: char,
    rhs: u32,
}

impl Condition {
    fn new(lhs: char, operator: char, rhs: u32) -> Self {
        Self { lhs, operator, rhs }
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}

impl Rule {
    fn new(condition: Option<Condition>, target: &str) -> Self {
        Self {
            condition,
            target: target.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_rule(rule: &str) -> Rule {
    let (condition, target) = rule.split_once(':').unwrap_or(("", rule));
    let condition = match condition {
        s if !s.is_empty() => Some(Condition::new(
            s.chars().next().unwrap(),
            s.chars().nth(1).unwrap(),
            s[2..].parse().unwrap(),
        )),
        _ => None,
    };
    Rule::new(condition, target)
}

fn parse_workflow(workflow: &str) -> (String, Vec<Rule>) {
    let (label, rules) = workflow.split_once('{').unwrap();
    let label = label.to_string();
    let rules = rules[..rules.len() - 1]
        .split(',')
        .map(parse_rule)
        .collect::<Vec<_>>();
    (label, rules)
}

fn is_accepted(workflows: &HashMap<String, Vec<Rule>>, part: &Part) -> bool {
    let mut rules = workflows.get("in").unwrap().iter();
    let mut rule = rules.next().unwrap();

    loop {
        if let Some(condition) = &rule.condition {
            let lhs = match condition.lhs {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!(),
            };
            if !(match condition.operator {
                '<' => lhs < condition.rhs,
                '>' => lhs > condition.rhs,
                _ => unreachable!(),
            }) {
                rule = rules.next().unwrap();
                continue;
            }
        }

        match &rule.target[..] {
            "A" => return true,
            "R" => return false,
            _ => {
                rules = workflows.get(&rule.target).unwrap().iter();
                rule = rules.next().unwrap();
                continue;
            }
        }
    }
}

fn process(
    workflows: &HashMap<String, Vec<Rule>>,
    potential_parts: &mut [(u32, u32)],
    target_workflow: &str,
) -> Option<u64> {
    let mut product = 0;
    let workflow = workflows.get(target_workflow).unwrap();

    for rule in workflow.iter() {
        let mut partition_1 = potential_parts.to_owned();

        if let Some(condition) = &rule.condition {
            let lhs = match condition.lhs {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            };
            let rhs = condition.rhs;
            let operator = condition.operator;
            match operator {
                '<' => {
                    partition_1[lhs].1 = min(partition_1[lhs].1, rhs - 1);
                    potential_parts[lhs].0 = max(potential_parts[lhs].0, rhs);
                }
                '>' => {
                    partition_1[lhs].0 = max(partition_1[lhs].0, rhs + 1);
                    potential_parts[lhs].1 = min(potential_parts[lhs].1, rhs);
                }
                _ => unreachable!(),
            };
        }

        if partition_1.iter().all(|(min, max)| min < max) {
            match rule.target.as_str() {
                "A" => {
                    let res = partition_1
                        .iter()
                        .map(|(min, max)| ((max - min) as u64) + 1)
                        .product::<u64>();

                    product += res;
                }
                "R" => (),
                _ => {
                    let result = process(workflows, &mut partition_1, &rule.target);
                    if let Some(result) = result {
                        product += result;
                    }
                }
            }
        }

        if potential_parts.iter().any(|(min, max)| min > max) {
            break;
        }
    }

    Some(product)
}

fn parse_part(part: &str) -> Part {
    let (x, m, a, s) = part[1..part.len() - 1]
        .split_once(',')
        .map(|(x, rest)| {
            let (m, rest) = rest.split_once(',').unwrap();
            let (a, s) = rest.split_once(',').unwrap();
            (x, m, a, s)
        })
        .unwrap();

    Part {
        x: x[2..x.len()].parse().unwrap(),
        m: m[2..m.len()].parse().unwrap(),
        a: a[2..a.len()].parse().unwrap(),
        s: s[2..s.len()].parse().unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let binding = input.replace("\r\n", "\n");
    let (workflows, parts) = binding.split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflows.lines().map(parse_workflow).collect();
    let parts: Vec<Part> = parts.lines().map(parse_part).collect();

    let sum_ratings = parts
        .iter()
        .filter_map(|part| {
            if is_accepted(&workflows, part) {
                Some(part.x + part.m + part.a + part.s)
            } else {
                None
            }
        })
        .sum();

    Some(sum_ratings)
}

pub fn part_two(input: &str) -> Option<u64> {
    let binding = input.replace("\r\n", "\n");
    let (workflows, _) = binding.split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflows.lines().map(parse_workflow).collect();
    let mut potential_parts = vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)];

    process(&workflows, &mut potential_parts, "in")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule_no_condition() {
        let rule = parse_rule("A");
        assert_eq!(rule.condition, None);
        assert_eq!(rule.target, "A");
    }

    #[test]
    fn test_parse_rule_with_condition() {
        let rule = parse_rule("a>1716:R");
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: 'a',
                operator: '>',
                rhs: 1716,
            })
        );
        assert_eq!(rule.target, "R");

        let rule = parse_rule("s<537:gd");
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: 's',
                operator: '<',
                rhs: 537,
            })
        );
        assert_eq!(rule.target, "gd");
    }

    #[test]
    fn test_parse_workflow() {
        let (label, rules) = parse_workflow("px{a<2006:qkq,m>2090:A,rfg}");

        assert_eq!(label, "px");
        assert_eq!(rules.len(), 3);

        let rule = &rules[0];
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: 'a',
                operator: '<',
                rhs: 2006,
            })
        );
        assert_eq!(rule.target, "qkq");

        let rule = &rules[1];
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: 'm',
                operator: '>',
                rhs: 2090,
            })
        );
        assert_eq!(rule.target, "A");

        let rule = &rules[2];
        assert_eq!(rule.condition, None);
        assert_eq!(rule.target, "rfg");
    }

    #[test]
    fn test_parse_part() {
        let part = parse_part("{x=2,m=3,a=0,s=0}");
        assert_eq!(
            part,
            Part {
                x: 2,
                m: 3,
                a: 0,
                s: 0,
            }
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
