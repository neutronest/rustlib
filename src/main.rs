extern crate rustlib;
use rustlib::graph;

fn main() {

    let s = "daze";
    println!("hello world, {}!", s);

    /* Graph Expertiment */
    let mut node_1 = graph::Node::new(1);
    let mut node_2 = graph::Node::new(2);
    let mut node_3 = graph::Node::new(3);

    node_1.add_adj_node(&node_2);
    node_1.add_adj_node(&node_3);
    node_2.add_adj_node(&node_1);


}
