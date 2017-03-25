use std::cell::{RefCell, Cell};
use std::rc::Rc;

/* version 1

pub struct Node<T> {

    inner_value: T,
    adj_node: Vec<Node<T>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        let node = Node {inner_value: value, adj_node: vec![]};
        node
    }

    pub fn add_adj_node(&mut self, other_node: Node<T>) {
        self.adj_node.push(other_node);
    }
}

pub struct Graph<T> {

    pub nodes: Vec<Node<T>>
}

impl<T> Graph<T> {
    pub fn new(other_nodes: Vec<Node<T>>) -> Self {
        Graph{nodes: other_nodes}
    }
}

 */


/* version 2 */

/*
pub struct Node<'a, T:'a> {

    inner_value: T,
    adj_node: Vec<&'a Node<'a, T>>
}


impl<'a, T> Node<'a, T> {
    pub fn new(value: T) -> Node<'a, T> {
        let node = Node {inner_value: value, adj_node: vec![]};
        node
    }

    pub fn add_adj_node(&'a mut self, other_node: &'a Node<T>) {
        self.adj_node.push(other_node);
    }
}

pub struct Graph<'a, T:'a> {

    pub nodes: Vec<Node<'a, T>>
}

impl<'a, T> Graph<'a, T> {
    pub fn new(other_nodes: Vec<Node<'a, T>>) -> Self {
        Graph{nodes: other_nodes}
    }
}
 */

/* version 3 */
type NodeRef<T> = Rc<RefCell<NodeElement<T>>>;
pub struct NodeElement<T> {
    pub inner_value: T,
    pub adj_nodes: Vec<NodeRef<T>>
}

pub struct Node<T>(pub NodeRef<T>);

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        let node_element = NodeElement {inner_value: value, adj_nodes: vec![]};
        Node(Rc::new(RefCell::new(node_element)))
    }

    pub fn add_adj_node(&self, other_node: &Node<T>) {

        let mut node_ref_res = self.0.try_borrow_mut();
        match node_ref_res {
            Ok(mut node_ref) =>  {
                node_ref.adj_nodes.push(other_node.0.clone());
            },
            Err(e) => {
                return;
            }
        }
        /*
        let adj_node_ref = self.adj_nodes[0].try_borrow_mut();
        match adj_node_ref {
            Ok(adj_node) => {
                adj_node.push(other_node.clone());
            },

            Err(e) => {
                return;
            }
        }
        */
    }
}

pub struct Graph<T> {
    pub nodes: Vec<Node<T>>
}

impl<T> Graph<T> {

    pub fn new(nodes_: Vec<Node<T>>) -> Self {
        Graph {nodes: nodes_ }
    }
}
