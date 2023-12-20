use num_integer::lcm;
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Clone)]
enum PulseType {
    High,
    Low,
}

struct Pulse<'a> {
    pulse_type: PulseType,
    source: &'a str,
    destination: &'a str,
}

impl<'a> Pulse<'a> {
    fn new(pulse_type: PulseType, source: &'a str, destination: &'a str) -> Self {
        Self {
            pulse_type,
            source,
            destination,
        }
    }
}

#[derive(Debug, Clone)]
enum CommsModule<'a> {
    FlipFlop {
        on_off: bool,
        destinations: Vec<&'a str>,
        id: &'a str,
    },
    Conjunction {
        inputs: HashMap<&'a str, PulseType>,
        destinations: Vec<&'a str>,
        id: &'a str,
    },
    Broadcaster {
        destinations: Vec<&'a str>,
        id: &'a str,
    },
}

impl<'a> CommsModule<'a> {
    fn has_destination(&self, id: &str) -> bool {
        match self {
            Self::FlipFlop { destinations, .. } => destinations.contains(&id),
            Self::Conjunction { destinations, .. } => destinations.contains(&id),
            Self::Broadcaster { destinations, .. } => destinations.contains(&id),
        }
    }

    fn signal(&mut self, pulse_queue: &mut VecDeque<Pulse<'a>>, pulse: &Pulse<'a>) {
        match self {
            Self::FlipFlop {
                on_off,
                destinations,
                id,
            } => {
                if pulse.pulse_type == PulseType::High {
                    return;
                }

                *on_off = !*on_off;

                if *on_off {
                    for destination in destinations.iter() {
                        //println!("{} -high-> {}", id, destination);
                        pulse_queue.push_back(Pulse::new(PulseType::High, id, destination));
                    }
                } else {
                    for destination in destinations.iter() {
                        //println!("{} -low-> {}", id, destination);
                        pulse_queue.push_back(Pulse::new(PulseType::Low, id, destination));
                    }
                }
            }
            Self::Conjunction {
                inputs,
                destinations,
                id,
            } => {
                inputs.insert(pulse.source, pulse.pulse_type.clone());

                if inputs.iter().all(|(_, pulse)| *pulse == PulseType::High) {
                    for destination in destinations {
                        pulse_queue.push_back(Pulse::new(PulseType::Low, id, destination));
                    }
                } else {
                    for destination in destinations {
                        pulse_queue.push_back(Pulse::new(PulseType::High, id, destination));
                    }
                }
            }
            Self::Broadcaster { destinations, id } => {
                for destination in destinations {
                    pulse_queue.push_back(Pulse::new(pulse.pulse_type.clone(), id, destination));
                }
            }
        }
    }
}

fn parse_module(line: &str) -> (&str, CommsModule) {
    Regex::new(r"([%&])?([a-z]+) -> (.*)")
        .unwrap()
        .captures(line)
        .map(|captures| {
            let id = captures.get(2).map(|m| m.as_str()).unwrap();
            let destinations = captures
                .get(3)
                .map(|m| m.as_str())
                .unwrap()
                .split(", ")
                .map(|s| s)
                .collect();
            let module = match captures.get(1).map(|m| m.as_str()) {
                Some("%") => CommsModule::FlipFlop {
                    on_off: false,
                    destinations,
                    id,
                },
                Some("&") => CommsModule::Conjunction {
                    inputs: HashMap::new(),
                    destinations,
                    id,
                },
                _ => CommsModule::Broadcaster { destinations, id },
            };

            (id, module)
        })
        .unwrap()
}

fn parse_input(input: &str) -> HashMap<&str, CommsModule> {
    let mut modules: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (id, module) = parse_module(line);
            (id, module)
        })
        .collect();

    let mut conjunctions: Vec<(&str, Vec<&str>)> = Vec::new();

    for (id, module) in modules.iter() {
        if matches!(module, CommsModule::Conjunction { .. }) {
            let inputs: Vec<&str> = modules
                .iter()
                .filter(|(_, m)| m.has_destination(id))
                .map(|(&id, _)| id)
                .collect();

            conjunctions.push((id, inputs));
        }
    }

    for (id, new_inputs) in conjunctions {
        match modules.get_mut(&id).unwrap() {
            CommsModule::Conjunction { inputs, .. } => {
                inputs.extend(new_inputs.iter().map(|&id| (id, PulseType::Low)));
            }
            _ => panic!("Expected conjunction"),
        }
    }

    modules
}

fn press_button(modules: &mut HashMap<&str, CommsModule>) -> (usize, usize) {
    let mut signals = VecDeque::from([Pulse::new(PulseType::Low, "button", "broadcaster")]);

    let mut low_counter = 0;
    let mut high_counter = 0;

    while !signals.is_empty() {
        let pulse = signals.pop_front().unwrap();

        if pulse.pulse_type == PulseType::Low {
            low_counter += 1;
        } else {
            high_counter += 1;
        }

        if !modules.contains_key(pulse.destination) {
            continue;
        }

        let module = modules.get_mut(pulse.destination).unwrap();

        module.signal(&mut signals, &pulse);
    }

    (low_counter, high_counter)
}

fn press_button2(modules: &mut HashMap<&str, CommsModule>, check: &str) -> bool {
    let mut signals = VecDeque::from([Pulse::new(PulseType::Low, "button", "broadcaster")]);

    while !signals.is_empty() {
        let pulse = signals.pop_front().unwrap();

        if pulse.source == check && pulse.destination == "lx" && pulse.pulse_type == PulseType::High
        {
            return true;
        }

        if !modules.contains_key(pulse.destination) {
            continue;
        }

        let module = modules.get_mut(pulse.destination).unwrap();

        module.signal(&mut signals, &pulse);
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules = parse_input(input);

    let mut lows = 0;
    let mut highs = 0;

    for _ in 0..1000 {
        let (low_count, high_count) = press_button(&mut modules);
        lows += low_count;
        highs += high_count;
    }

    Some(lows * highs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let modules = parse_input(input);

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
}
