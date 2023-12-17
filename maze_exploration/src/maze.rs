use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
pub struct Node {
    pub label: String,
    pub connections: Vec<String>,
}
pub struct Maze {
    nodes: Vec<Node>,
    exits: Vec<String>, 
}

impl Maze {
    pub fn new() -> Self {
        Maze {
            nodes: vec![
                Node { label: "A".to_string(), connections: vec!["B".to_string()] },
                Node { label: "B".to_string(), connections: vec!["A".to_string(), "C".to_string()] },
                Node { label: "C".to_string(), connections: vec!["B".to_string()] },
            ],
            exits: vec!["C".to_string()],
        }
    }
    pub fn get_start_nodes(&self) -> Vec<Node> {
        vec![self.nodes[0].clone()]
    }
    pub fn explore(&self, start_node: &Node, trace: &mut Vec<String>, exit_found: &Arc<Mutex<bool>>) {
        if *exit_found.lock().unwrap() {
            return; 
        }
        trace.push(start_node.label.clone());
        if self.exits.contains(&start_node.label) {
            *exit_found.lock().unwrap() = true;
            return;
        }
        for next_label in &start_node.connections {
            if let Some(next_node) = self.nodes.iter().find(|n| &n.label == next_label) {
                if !trace.contains(&next_node.label) {
                    self.explore(next_node, trace, exit_found);
                }
            }
        }
        thread::yield_now(); 
    }
}