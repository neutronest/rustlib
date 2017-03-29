use std::string;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::ops::Range;

type NodeRef<T> = Rc<RefCell<Node<T>>>;


/*
The basic node foundation of SkipList
key-value is just k-v for search, get and put
forward_nodes is all the next nodes with different level that it forward
*/
pub struct Node<T> {
    pub key: String,
    pub value: T,
    pub forward_nodes: Vec<NodeRef<T>>
}

impl<T> Node<T> where T: Ord + Copy + Clone {

    pub fn new(key_: String, value_: T) -> Node<T> {

        Node{key: key_, value: value_, forward_nodes: vec![]}
    }


    pub fn get_value(&self) -> T {
        return self.value.clone();
    }
}

pub struct SkipList<T> {

    pub root_opt: Option<NodeRef<T>>,
    pub level: usize
}

impl<T> SkipList<T> where
    T: Ord + Copy + Clone  {

    pub fn get(&self, key_: String) -> Option<T> {

        match self.root_opt {
            Some(ref root_ref) => {
                let mut cur_node = root_ref.borrow(); 
                let mut cur_level = self.level;
                while cur_level >= 1 {
                    if cur_node.key == key_ {
                        return Some(cur_node.value);
                    } else if  cur_node.key < key_ {

                        // side condition
                        // check if the last node
                        if cur_node.forward_nodes.len() == 0 {
                            cur_level -= 1;
                        } else if cur_node.forward_nodes[cur_level].borrow().key < key_ {
                            // move to the next node
                            let mut cur_node = cur_node.forward_nodes[cur_level].borrow();
                        }
                    }
                }
                return None;
            },
            None => {
                // empty skiplist
                return None;
            }
        }

        return None;
    }
}
