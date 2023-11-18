use std::collections::HashSet;
use rand::{seq::IteratorRandom, Rng};
use petgraph::{Directed, Graph};

pub fn get_sample_graph(min_nodes: u32, max_nodes: u32) -> (Graph<(), i32>, u32, u32) {
    let mut rng = rand::thread_rng();
    let node_number = rng.gen_range((min_nodes - 1)..max_nodes);
    let number_of_pathes = rng.gen_range(3..10);
    let source = 0;
    let sink = node_number as u32;
    let mut all_pathes = Vec::new();

    for _ in 0..number_of_pathes {
        let mut path_vec = Vec::new();
        path_vec.push(source);
        let number_of_nodes_for_path = rng.gen_range(3..node_number);
        let indices =
            (1..node_number - 1).choose_multiple(&mut rng, number_of_nodes_for_path as usize);
        all_pathes.push(indices);
    }

    let mut graph: Graph<(), i32, Directed> = Graph::new();

    let mut edges: HashSet<(u32, u32)> = HashSet::new();

    for path in all_pathes {
        let mut iter = path.into_iter();
        let mut node = iter.next().unwrap() as u32;
        edges.insert((source, node));
        while let Some(target_node) = iter.next() {
            edges.insert((node, target_node as u32));
            node = target_node as u32;
        }
        edges.insert((node, sink));
    }

    let edge_vec: Vec<(u32, u32, i32)> = edges
        .into_iter()
        .map(|elem| (elem.0, elem.1, rng.gen_range(1..20)))
        .collect();

    graph.extend_with_edges(edge_vec);

    (graph, source, sink)
}