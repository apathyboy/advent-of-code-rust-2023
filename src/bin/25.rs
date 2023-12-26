use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::UnGraph;
use rustworkx_core::Result;
use std::collections::HashMap;

advent_of_code::solution!(25);

fn parse_graph(input: &str) -> UnGraph<&str, u32> {
    let mut graph = UnGraph::new_undirected();

    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();

    for line in input.lines() {
        let (node_str, connections) = line.split_once(": ").unwrap();
        let connections: Vec<_> = connections.split(' ').collect();

        if !nodes.contains_key(node_str) {
            nodes.insert(node_str, graph.add_node(node_str));
        }

        for &connection_str in &connections {
            let key = if node_str < connection_str {
                [node_str, connection_str]
            } else {
                [connection_str, node_str]
            };

            if !nodes.contains_key(connection_str) {
                nodes.insert(connection_str, graph.add_node(connection_str));
            }

            if !edges.contains_key(&key) {
                edges.insert(
                    key,
                    graph.add_edge(nodes[node_str], nodes[connection_str], 1),
                );
            }
        }
    }

    graph
}

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_graph(input);

    let min: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, partition) = min.unwrap().unwrap();

    Some(partition.len() * (graph.node_count() - partition.len()))
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
