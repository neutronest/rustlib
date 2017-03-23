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
