use std::time::Instant;
use petgraph::{Directed, Graph};

mod path_tracker;
mod solver;
mod plot_function;
mod sample_graph;

use plot_function::plot;
use solver::EdmondsKarpSolver;
use sample_graph::get_sample_graph;

fn main() {
    let source = 5;
    let sink = 4;

    let graph = Graph::<(), i32, Directed>::from_edges(&[
        (0, 1, 2),
        (0, 2, 4),
        (0, 3, 8),
        (1, 3, 9),
        (3, 2, 6),
        (source, 0, 10),
        (source, 1, 10),
        (2, sink, 10),
        (3, sink, 10),
    ]);
    let source_2 = 10;
    let sink_2 = 11;

    let graph_2 = Graph::<(), i32, Directed>::from_edges(&[
        (source_2, 0, 10),
        (source_2, 1, 5),
        (source_2, 2, 10),
        (0, 3, 10),
        (1, 2, 10),
        (2, 5, 15),
        (3, 1, 2),
        (3, 6, 15),
        (4, 1, 15),
        (4, 3, 3),
        (5, 4, 4),
        (5, 8, 10),
        (6, 7, 10),
        (7, 4, 10),
        (7, 5, 7),
        (6, sink_2, 15),
        (8, sink_2, 10),
    ]);
    let mut solver = EdmondsKarpSolver::new();
    let mut series = Vec::new();

    let min_number_of_nodes = 5;
    let max_number_of_nodes = 25;

    for _ in 0..1000 {
        let g = get_sample_graph(min_number_of_nodes, max_number_of_nodes);
        let now = Instant::now();
        let _max_flow = solver.solve(&g.0, g.1, g.2);
        let elapsed = now.elapsed();
        series.push((elapsed.as_nanos() as f32 / 1000_000.0, g.2 + 1));
        //println!("{}: {}", i, max_flow);
    }

    plot(series, max_number_of_nodes, min_number_of_nodes).unwrap();
}
