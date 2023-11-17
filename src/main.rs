use petgraph::{Directed, Graph};

mod path_tracker;
mod solver;

use solver::EdmondsKarpSolver;

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
    let max_flow = solver.solve(&graph_2, source_2, sink_2);
    println!("{}", max_flow);
}
