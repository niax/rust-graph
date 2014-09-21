extern crate graph;

use graph::{Graph};


fn main() {
    let mut g = Graph::new();
    let nodes = g.insert_all([0i, 1, 2, 3]);
    g.connect_all([
        (nodes[0], nodes[1], 5u),
        (nodes[1], nodes[2], 5u),
        (nodes[0], nodes[3], 10u),
        (nodes[1], nodes[3], 3u)
    ]);

    println!("From 0 to 3: {}", g.shortest_path(nodes[0], nodes[3]).unwrap().path);
}
