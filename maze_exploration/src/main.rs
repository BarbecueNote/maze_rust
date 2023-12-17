mod maze;

use maze::{Maze};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let maze = Arc::new(Mutex::new(Maze::new())); 
    let explored = Arc::new(Mutex::new(Vec::new()));
    let exit_found = Arc::new(Mutex::new(false));
    let start_nodes = maze.lock().unwrap().get_start_nodes();

    let mut handles = vec![];
    for start_node in start_nodes {
        let maze_clone = Arc::clone(&maze);
        let explored_clone = Arc::clone(&explored);
        let exit_found_clone = Arc::clone(&exit_found);

        let handle = thread::spawn(move || {
            maze_clone.lock().unwrap().explore(start_node, &explored_clone, &exit_found_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Explored nodes: {:?}", *explored.lock().unwrap());
}
