#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Node = usize;
type Weight = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Weight)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Weight)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Weight)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list.entry(source).or_insert_with(|| Vec::new());

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Step {
    weight: Weight,
    node: Node,
    path: Vec<Node>,
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Weight)> {
    let mut travel_weights = g
        .nodes
        .iter()
        .map(|n| {
            if *n == start {
                (*n, 0)
            } else {
                (*n, usize::max_value())
            }
        })
        .collect::<HashMap<_, _>>();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Step {
        weight: 0,
        node: start,
        path: vec![],
    });

    while let Some(Step {
        weight,
        node,
        mut path,
    }) = priority_queue.pop()
    {
        if node == goal {
            path.push(node);
            return Some((path, weight));
        }

        if let Some(vertices) = &g.edges.get(&node) {
            for &(v_destination, v_weight) in vertices.iter() {
                if v_weight < travel_weights[&v_destination] {
                    let mut new_path = path.clone();
                    new_path.push(node);
                    priority_queue.push(Step {
                        weight: weight + v_weight,
                        node: v_destination,
                        path: new_path,
                    });

                    let old_weight = travel_weights.get_mut(&v_destination).unwrap();
                    *old_weight = v_weight;
                }
            }
        }
    }

    None
}

#[test]
fn large_graph() {
    let edge_list = include!("challenge_16_large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24);
}
