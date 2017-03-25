extern crate rustlib;
use rustlib::graph;

fn main() {

    let s = "daze";
    println!("hello world, {}!", s);

    /* Graph Expertiment */
    let node_1 = graph::Node::new(1);
    let node_2 = graph::Node::new(2);
    let node_3 = graph::Node::new(3);

    node_1.add_adj_node(&node_2);
    node_1.add_adj_node(&node_3);
    node_2.add_adj_node(&node_1);
    node_3.add_adj_node(&node_1);

    let mut g = graph::Graph::new(vec![node_1, node_2, node_3]);
    for gnode in g.nodes.iter().map(|n| n.0.borrow()) {
        let value = gnode.inner_value;
        let adj_values = gnode
            .adj_nodes.iter()
            .map(|x| x.borrow().inner_value).collect::<Vec<_>>();
        println!("node {} is connected to: {:?}", value, adj_values);
    }
    
}
