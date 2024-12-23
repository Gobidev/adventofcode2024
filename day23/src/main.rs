use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::algo::maximal_cliques;
use petgraph::prelude::*;

fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

fn build_graph(edges: Vec<(String, String)>) -> UnGraph<String, ()> {
    let mut graph = UnGraph::new_undirected();
    let mut node_indices = HashMap::new();

    for (a, b) in edges {
        let from_index = *node_indices
            .entry(a.clone())
            .or_insert_with(|| graph.add_node(a));
        let to_index = *node_indices
            .entry(b.clone())
            .or_insert_with(|| graph.add_node(b));
        graph.add_edge(from_index, to_index, ());
    }

    graph
}

fn part1(graph: &UnGraph<String, ()>) -> usize {
    let mut cliques3 = HashSet::new();
    for edge in graph.edge_references() {
        for &w in graph
            .neighbors(edge.source())
            .collect::<HashSet<_>>()
            .intersection(&graph.neighbors(edge.target()).collect::<HashSet<_>>())
        {
            let mut clique3 = [edge.source(), edge.target(), w];
            clique3.sort_unstable();
            cliques3.insert(clique3);
        }
    }
    cliques3
        .iter()
        .filter(|clique| {
            clique
                .iter()
                .any(|n_idx| graph.node_weight(*n_idx).unwrap().starts_with("t"))
        })
        .count()
}

fn part2(graph: &Graph<String, (), Undirected>) -> String {
    maximal_cliques(graph)
        .iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .iter()
        .map(|n_idx| graph.node_weight(*n_idx).unwrap().to_string())
        .sorted_unstable()
        .join(",")
}

fn main() {
    let input = parse(include_str!("../input.txt"));
    let graph = build_graph(input);
    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}
