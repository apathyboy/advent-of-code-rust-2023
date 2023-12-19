use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug, PartialEq)]
struct Condition {
    lhs: String,
    operator: char,
    rhs: u32,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    target: String,
}

impl Rule {
    fn new(condition: Option<Condition>, target: String) -> Self {
        Self { condition, target }
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
    let condition = if condition.contains('<') || condition.contains('>') {
        let (lhs, rhs) = condition
            .split_once('<')
            .unwrap_or_else(|| condition.split_once('>').unwrap());
        let operator = if condition.contains('<') { '<' } else { '>' };
        let rhs = rhs.parse().unwrap();
        Some(Condition {
            lhs: lhs.to_string(),
            operator,
            rhs,
        })
    } else {
        None
    };
    Rule::new(condition, target.to_string())
}

fn parse_workflow(workflow: &str) -> (String, Vec<Rule>) {
    let (label, rules) = workflow.split_once('{').unwrap();
    let label = label.to_string();
    let rules = rules
        .split_once('}')
        .unwrap()
        .0
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
            let lhs = match condition.lhs.as_str() {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => unreachable!(),
            };
            let rhs = condition.rhs;
            let operator = condition.operator;
            if !(match operator {
                '<' => lhs < rhs,
                '>' => lhs > rhs,
                _ => unreachable!(),
            }) {
                rule = rules.next().unwrap();
                continue;
            }
        }

        match rule.target.as_str() {
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

pub fn part_two(input: &str) -> Option<u32> {
    let binding = input.replace("\r\n", "\n");
    let (workflows, _) = binding.split_once("\n\n").unwrap();

    let workflows: HashMap<_, _> = workflows.lines().map(parse_workflow).collect();

    // start with a potential part with ranges of 0..=4000 for each attribute (x, m, a, s)
    // then start at the workflow "in" and follow the rules until we reach "A" or "R"
    // branch the potential part at each rule, limitng the ranges of the attributes based on the condition
    // if we reach "A" then we have a valid part, if we reach "R" then we have an invalid part
    // if we reach a rule that has no condition, then we branch the potential part into two parts
    // one part with the rule's target and one part without the rule's target
    // we continue until we have no more rules to follow
    // then we count all potential rules that have "A" as a target

    let mut potential_parts: Vec<(u32, u32)> = vec![(0, 4000), (0, 4000), (0, 4000), (0, 4000)];
    let mut valid_parts: Vec<(u32, u32)> = vec![];

    let mut rules = workflows.get("in").unwrap().iter();
    let mut rule = rules.next().unwrap();

    None
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
                lhs: "a".to_string(),
                operator: '>',
                rhs: 1716,
            })
        );
        assert_eq!(rule.target, "R");

        let rule = parse_rule("s<537:gd");
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: "s".to_string(),
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
                lhs: "a".to_string(),
                operator: '<',
                rhs: 2006,
            })
        );
        assert_eq!(rule.target, "qkq");

        let rule = &rules[1];
        assert_eq!(
            rule.condition,
            Some(Condition {
                lhs: "m".to_string(),
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
        assert_eq!(result, None);
    }
}
