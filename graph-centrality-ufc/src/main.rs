use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::collections::{HashMap, VecDeque};
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}
/**
    Betweenness centrality is a way of detecting the amount of influence a node has over
    the flow of information in a graph. It is often used to find nodes that serve as a bridge
    from one part of a graph to another. The algorithm calculates shortest paths between
    all pairs of nodes in a graph. Each node receives a score, based on the number of
    shortest paths that pass through the node. Nodes that more frequently lie on shortest
    paths between other nodes will have higher betweenness centrality scores.
    Implementation using Brande's algorithm.
    */
fn betweenness_centrality(graph: &UnGraph<&Fighter, f32>) -> HashMap<NodeIndex, f32> {
    let mut centrality = HashMap::new();
    for node in graph.node_indices() {
        centrality.insert(node, 0.0);
    }
    let mut stack = Vec::new();
    let mut pred: HashMap<NodeIndex, Vec<NodeIndex>> = HashMap::new();
    let mut sigma: HashMap<NodeIndex, usize> = HashMap::new(); //No. of shortest paths
    let mut dist: HashMap<NodeIndex, isize> = HashMap::new(); //Distance from source
    // For each source node, compute shortest paths and number of paths to each node.
    for v in graph.node_indices() {
        pred.insert(v, Vec::new());
        sigma.insert(v, 0);
        dist.insert(v, -1);
    }
    // sigma.insert(s, 1);
    // dist.insert(s, 0);

    let mut queue = VecDeque::new();
    // queue.push_back(s);

    // BFS to find shortest paths from s
    while let Some(v) = queue.pop_front() {
        stack.push(v);
    }
    centrality
}
fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        let betweenness = betweenness_centrality(&graph); 
        println!("The closeness centrality of {} is {:.2}", name, closeness);
        // Challenge 1: Pritn the betweenness centrality of each fighter
        println!("The betweenness centrality of {} is {:.2}", name,betweenness);

        // Explanation
        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");
    }
}
