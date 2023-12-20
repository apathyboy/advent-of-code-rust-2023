use num_integer::lcm;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum CommsModule {
    FlipFlop {
        on_off: bool,
        destinations: Vec<String>,
        id: String,
    },
    Conjunction {
        inputs: HashMap<String, Pulse>,
        destinations: Vec<String>,
        id: String,
    },
    Broadcaster {
        destinations: Vec<String>,
        id: String,
    },
}

impl CommsModule {
    fn has_destination(&self, id: &str) -> bool {
        match self {
            Self::FlipFlop { destinations, .. } => destinations.contains(&id.to_string()),
            Self::Conjunction { destinations, .. } => destinations.contains(&id.to_string()),
            Self::Broadcaster { destinations, .. } => destinations.contains(&id.to_string()),
        }
    }

    fn signal(
        &mut self,
        source: &str,
        pulse_queue: &mut VecDeque<(String, String, Pulse)>,
        pulse: Pulse,
    ) {
        match self {
            Self::FlipFlop {
                on_off,
                destinations,
                id,
            } => {
                if pulse == Pulse::High {
                    return;
                }

                *on_off = !*on_off;

                if *on_off {
                    for destination in destinations.iter() {
                        //println!("{} -high-> {}", id, destination);
                        pulse_queue.push_back((id.clone(), destination.clone(), Pulse::High));
                    }
                } else {
                    for destination in destinations.iter() {
                        //println!("{} -low-> {}", id, destination);
                        pulse_queue.push_back((id.clone(), destination.clone(), Pulse::Low));
                    }
                }
            }
            Self::Conjunction {
                inputs,
                destinations,
                id,
            } => {
                inputs.insert(source.to_string(), pulse.clone());

                if inputs.iter().all(|(_, pulse)| *pulse == Pulse::High) {
                    for destination in destinations {
                        //println!("{} -low-> {}", id, destination);
                        pulse_queue.push_back((id.clone(), destination.clone(), Pulse::Low));
                    }
                } else {
                    for destination in destinations {
                        //println!("{} -high-> {}", id, destination);
                        pulse_queue.push_back((id.clone(), destination.clone(), Pulse::High));
                    }
                }
            }
            Self::Broadcaster { destinations, id } => {
                for destination in destinations {
                    //println!(
                    //    "{} -{}-> {}",
                    //    id,
                    //    match pulse {
                    //        Pulse::High => "high",
                    //        Pulse::Low => "low",
                    //    },
                    //    destination
                    //);
                    pulse_queue.push_back((id.clone(), destination.clone(), pulse.clone()));
                }
            }
        }
    }
}

fn parse_module(line: &str) -> (String, CommsModule) {
    Regex::new(r"([%&])?([a-z]+) -> (.*)")
        .unwrap()
        .captures(line)
        .map(|captures| {
            let id = captures.get(2).map(|m| m.as_str()).unwrap().to_string();
            let module = match captures.get(1).map(|m| m.as_str()) {
                Some("%") => CommsModule::FlipFlop {
                    on_off: false,
                    destinations: captures
                        .get(3)
                        .map(|m| m.as_str())
                        .unwrap()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                    id: id.clone(),
                },
                Some("&") => CommsModule::Conjunction {
                    inputs: HashMap::new(),
                    destinations: captures
                        .get(3)
                        .map(|m| m.as_str())
                        .unwrap()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                    id: id.clone(),
                },
                _ => CommsModule::Broadcaster {
                    destinations: captures
                        .get(3)
                        .map(|m| m.as_str())
                        .unwrap()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                    id: id.clone(),
                },
            };

            (id, module)
        })
        .unwrap()
}

fn press_button(modules: &mut HashMap<String, CommsModule>) -> (usize, usize) {
    let mut signals =
        VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);

    let mut low_counter = 0;
    let mut high_counter = 0;

    //println!("button -low-> broadcaster");

    while !signals.is_empty() {
        let (from, to, pulse) = signals.pop_front().unwrap();
        if pulse == Pulse::Low {
            low_counter += 1;
        } else {
            high_counter += 1;
        }

        if !modules.contains_key(to.as_str()) {
            continue;
        }

        let module = modules.get_mut(to.as_str()).unwrap();

        module.signal(&from, &mut signals, pulse);
    }

    (low_counter, high_counter)
}

fn press_button2(modules: &mut HashMap<String, CommsModule>, check: &str) -> bool {
    let mut signals =
        VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);

    while !signals.is_empty() {
        let (from, to, pulse) = signals.pop_front().unwrap();

        if from == check && to == "lx" && pulse == Pulse::High {
            return true;
        }

        if !modules.contains_key(to.as_str()) {
            continue;
        }

        let module = modules.get_mut(to.as_str()).unwrap();

        module.signal(&from, &mut signals, pulse);
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (id, module) = parse_module(line);
            (id, module)
        })
        .collect();

    let mut conjunctions: Vec<(String, Vec<String>)> = Vec::new();

    for (id, module) in modules.iter() {
        if matches!(module, CommsModule::Conjunction { .. }) {
            let inputs: Vec<String> = modules
                .iter()
                .filter(|(_, m)| m.has_destination(id))
                .map(|(id, _)| id.clone())
                .collect();

            conjunctions.push((id.clone(), inputs));
        }
    }

    for (id, new_inputs) in conjunctions {
        match modules.get_mut(&id).unwrap() {
            CommsModule::Conjunction { inputs, .. } => {
                inputs.extend(
                    new_inputs
                        .iter()
                        .map(|id: &String| (id.clone(), Pulse::Low)),
                );
            }
            _ => panic!("Expected conjunction"),
        }
    }

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        let (low_count, high_count) = press_button(&mut modules);
        lows += low_count;
        highs += high_count;

        //println!("");
    }

    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (id, module) = parse_module(line);
            (id, module)
        })
        .collect();

    let mut conjunctions: Vec<(String, Vec<String>)> = Vec::new();

    for (id, module) in modules.iter() {
        if matches!(module, CommsModule::Conjunction { .. }) {
            let inputs: Vec<String> = modules
                .iter()
                .filter(|(_, m)| m.has_destination(id))
                .map(|(id, _)| id.clone())
                .collect();

            conjunctions.push((id.clone(), inputs));
        }
    }

    for (id, new_inputs) in conjunctions {
        match modules.get_mut(&id).unwrap() {
            CommsModule::Conjunction { inputs, .. } => {
                inputs.extend(
                    new_inputs
                        .iter()
                        .map(|id: &String| (id.clone(), Pulse::Low)),
                );
            }
            _ => panic!("Expected conjunction"),
        }
    }

    let tmp = ["cl", "rp", "lb", "nj"]
        .into_par_iter()
        .map(|id| {
            let mut counter: usize = 1;
            let mut modules = modules.clone();

            while !press_button2(&mut modules, id) {
                counter += 1;
            }

            counter
        })
        .collect::<Vec<usize>>();

    tmp.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32_000_000));
    }

    #[test]
    fn test_part_on_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11_687_500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
