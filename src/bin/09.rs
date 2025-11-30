use std::collections::HashMap;

use itertools::Itertools;
use petgraph::graph::NodeIndex;
use petgraph::{self, Graph, Undirected};
advent_of_code::solution!(9);

fn parse_input(
    input: &str,
) -> (
    petgraph::Graph<&str, u64, petgraph::Undirected>,
    HashMap<String, NodeIndex>,
) {
    let mut graph: Graph<&str, u64, Undirected> = Graph::new_undirected();
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let (left, dist) = line.split_once(" = ").unwrap();
        let (fr, to) = left.split_once(" to ").unwrap();
        let dist_parsed = dist.parse::<u64>().unwrap();

        let fr_idx: NodeIndex = *nodes
            .entry(fr.to_owned())
            .or_insert_with(|| graph.add_node(fr));
        let to_idx: NodeIndex = *nodes
            .entry(to.to_owned())
            .or_insert_with(|| graph.add_node(to));
        graph.add_edge(fr_idx, to_idx, dist_parsed);
    }
    (graph, nodes)
}

fn path_distance(graph: &Graph<&str, u64, Undirected>, path: &[NodeIndex]) -> u64 {
    path.windows(2)
        .map(|w| {
            let edge = graph.find_edge(w[0], w[1]).unwrap();
            *graph.edge_weight(edge).unwrap()
        })
        .sum()
}

fn solve(input: &str) -> (u64, u64) {
    let (graph, nodes) = parse_input(input);
    let node_indices: Vec<NodeIndex> = nodes.values().copied().collect();
    let mut min_dist = u64::MAX;
    let mut max_dist = 0;
    for perm in node_indices
        .iter()
        .copied()
        .permutations(node_indices.len())
    {
        let dist = path_distance(&graph, &perm);
        min_dist = min_dist.min(dist);
        max_dist = max_dist.max(dist);
    }
    (min_dist, max_dist)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (min_dist, _max_dist) = solve(input);
    Some(min_dist)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_min_dist, max_dist) = solve(input);
    Some(max_dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
