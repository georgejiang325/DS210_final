//import modules
use petgraph::graph::Graph;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::prelude::NodeIndex;

pub fn create_graph() -> Graph<i32, ()> {
    let file = File::open("roadNet-CA.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    let mut node_indices: HashMap<i32, NodeIndex> = HashMap::new();

    let mut line_buffer = String::new();

    //limited to first 8000 lines as the file is too big
    for line in reader.lines().take(8000) {
        line_buffer.clear();
        line_buffer.push_str(&line.unwrap());
        let nodes: Vec<&str> = line_buffer.split_whitespace().collect();
        let node1 = *node_indices
            .entry(nodes[0].parse().unwrap())
            .or_insert_with(|| graph.add_node(nodes[0].parse().unwrap()));
        let node2 = *node_indices
            .entry(nodes[1].parse().unwrap())
            .or_insert_with(|| graph.add_node(nodes[1].parse().unwrap()));
        graph.add_edge(node1, node2, ());
    }

    graph
}


    
