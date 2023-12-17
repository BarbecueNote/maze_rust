use std::sync::{Arc, Mutex};

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

    pub fn explore(&self, node: Node, explored: &Arc<Mutex<Vec<Node>>>, exit_found: &Arc<Mutex<bool>>) {
        let mut explored_guard = explored.lock().unwrap();
        if explored_guard.iter().any(|n| n.label == node.label) || *exit_found.lock().unwrap() {
            return; 
        }

        explored_guard.push(node.clone());

        if self.exits.contains(&node.label) {
            *exit_found.lock().unwrap() = true;
            return;
        }

        drop(explored_guard);

        for next_label in node.connections.iter() {
            if let Some(next_node) = self.nodes.iter().find(|n| &n.label == next_label) {
                self.explore(next_node.clone(), explored, exit_found);
            }
        }
    }
}
