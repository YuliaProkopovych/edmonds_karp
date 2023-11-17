use crate::path_tracker::PathTracker;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use std::collections::{HashSet, VecDeque};

pub struct EdmondsKarpSolver {
    residual_graph: Option<Graph<(), FlowValue>>,
    target: Option<u32>,
    source: Option<u32>,
}

#[derive(Debug)]
struct FlowValue {
    capacity: i32,
    flow: i32,
}

impl FlowValue {
    fn remaining_capacity(&self) -> i32 {
        self.capacity - self.flow
    }
}

impl EdmondsKarpSolver {
    pub fn new() -> Self {
        EdmondsKarpSolver {
            residual_graph: None,
            target: None,
            source: None,
        }
    }

    pub fn solve(&mut self, graph: &Graph<(), i32>, source: u32, target: u32) -> i32 {
        self.residual_graph = Some(Self::generate_residual_graph(graph));
        self.target = Some(target);
        self.source = Some(source);
        self.calculate_max_flow()
    }

    fn target(&self) -> u32 {
        self.target.unwrap()
    }
    fn source(&self) -> u32 {
        self.source.unwrap()
    }
    fn graph_mut(&mut self) -> &mut Graph<(), FlowValue> {
        self.residual_graph.as_mut().unwrap()
    }
    fn graph(&self) -> &Graph<(), FlowValue> {
        self.residual_graph.as_ref().unwrap()
    }

    fn generate_residual_graph(initial_graph: &Graph<(), i32>) -> Graph<(), FlowValue> {
        let mut graph = initial_graph.clone();
        graph.extend_with_edges(
            initial_graph
                .raw_edges()
                .into_iter()
                .map(|e| (e.target(), e.source(), 0)),
        );

        let mut residual_graph: Graph<(), FlowValue> = Graph::new();

        residual_graph.extend_with_edges(graph.edge_references().map(|edge| {
            (
                edge.source(),
                edge.target(),
                FlowValue {
                    capacity: *edge.weight(),
                    flow: 0,
                },
            )
        }));
        residual_graph
    }

    fn calculate_max_flow(&mut self) -> i32 {
        let mut max_flow = 0;
        let mut flow = self.calculate_augmenting_path_flow();

        while flow != 0 {
            max_flow += flow;
            flow = self.calculate_augmenting_path_flow();
        }

        max_flow
    }

    fn calculate_augmenting_path_flow(&mut self) -> i32 {
        let mut visited: HashSet<u32> = HashSet::new(); //track visited nodes to avoid cycles

        let mut path: PathTracker<Graph<(), FlowValue>> = PathTracker::new();
        let mut queue: VecDeque<u32> = VecDeque::new();
        visited.insert(self.source());
        queue.push_front(self.source());

        //find flow path
        while !queue.is_empty() {
            let node_index = queue.pop_front().unwrap();
            if node_index == self.target() {
                break;
            }

            for edge in self.graph().edges(node_index.into()) {
                let capacity = edge.weight().remaining_capacity();
                let target_index = edge.target().index() as u32;
                if capacity > 0 && !visited.contains(&target_index) {
                    visited.insert(target_index);
                    path.set_predecessor(edge.target(), edge.source());
                    queue.push_back(target_index);
                }
            }
        }

        // if no path from source to target found, exit
        let path_vec = path.reconstruct_path_to(self.target().into());
        if path_vec.len() <= 1 {
            return 0;
        }

        //calculate bottleneck value
        let mut path_vec_iter = path_vec.iter().peekable();
        let mut bottleneck_value = i32::MAX;
        while let Some(first_node) = path_vec_iter.next() {
            if let Some(second_node) = path_vec_iter.peek() {
                let edge_index = self
                    .graph()
                    .find_edge((*first_node).into(), (**second_node).into())
                    .unwrap();
                let next_bottleneck_value = self.graph()[edge_index].remaining_capacity();
                bottleneck_value = bottleneck_value.min(next_bottleneck_value);
            }
        }

        self.augment_path(&path_vec, bottleneck_value);

        bottleneck_value
    }
    fn augment_path(&mut self, path: &Vec<NodeIndex>, bottleneck_value: i32) {
        let mut path_vec_iter = path.iter().peekable();

        while let Some(first_node) = path_vec_iter.next() {
            if let Some(second_node) = path_vec_iter.peek() {
                let edge = self
                    .graph()
                    .find_edge(*first_node, **second_node)
                    .expect("Incorect path, no edge found");
                let residual_edge = self
                    .graph()
                    .find_edge(**second_node, *first_node)
                    .expect("Incorect path, no edge found");

                let flow_value = self.graph_mut().edge_weight_mut(edge).unwrap();
                flow_value.flow += bottleneck_value;
                let residual_flow_value = self.graph_mut().edge_weight_mut(residual_edge).unwrap();
                residual_flow_value.flow -= bottleneck_value;
            }
        }
    }
}
