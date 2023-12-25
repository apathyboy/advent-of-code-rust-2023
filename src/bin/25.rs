use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(25);

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let (node, connections) = line.split_once(": ").unwrap();
        let connections: HashSet<_> = connections.split(' ').collect();

        let parent = graph.entry(node).or_insert(HashSet::new());

        for &connection in &connections {
            parent.insert(connection);
        }

        for &connection in &connections {
            let child = graph.entry(connection).or_default();
            child.insert(node);
        }
    }

    graph
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse(input);

    let mut freq = HashMap::new();

    for &start in graph.keys() {
        let mut queue = VecDeque::from([start]);
        let mut visited = HashSet::from([start]);

        while let Some(node) = queue.pop_front() {
            for &next in &graph[node] {
                if visited.insert(next) {
                    let key = if node < next {
                        [node, next]
                    } else {
                        [next, node]
                    };

                    let entry = freq.entry(key).or_insert(0);
                    *entry += 1;

                    queue.push_back(next);
                }
            }
        }
    }

    let mut order: Vec<_> = freq.iter().collect();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();

    let cut: Vec<_> = order.iter().take(3).map(|p| *p.0).collect();
    let start = *graph.keys().next().unwrap();

    let mut todo = VecDeque::new();
    todo.push_back(start);

    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some(pos) = todo.pop_front() {
        for &next in &graph[pos] {
            let key = if pos < next { [pos, next] } else { [next, pos] };

            if cut.contains(&key) {
                continue;
            }

            if seen.insert(next) {
                todo.push_back(next);
            }
        }
    }

    Some(seen.len() * (graph.len() - seen.len()))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
