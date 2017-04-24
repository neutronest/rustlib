
//use config;

use std::string;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::ops::Range;
use rand;
use core::mem;
use core::ptr::{self, Shared};
use core::marker::PhantomData;
use core::fmt;

type NodeBox<K, T> = Option<Shared<SkipNode<K, T>>>;

pub struct SkipNode<K, T> {

    pub key: K,
    pub value: T,
    pub next: Vec<NodeBox<K, T>>,

}

impl<K, T> SkipNode<K, T>
    where K: Clone + Ord + PartialOrd + Eq + fmt::Display,
          T: Clone{


    pub fn new(key_: K, value_: T, level_: usize) -> Self {

        let mut v = Vec::new();
        for i in 0..level_ {
            v.push(None);
        }
        SkipNode {
            key: key_,
            value: value_,
            next: v,
        }
    }

    pub fn next(&self, level_: usize) -> NodeBox<K, T> {

        let len = self.next.len();
        if level_ >= len {
            return None;
        }
        return self.next[level_];
    }

    pub fn modify_next(&mut self, level_: usize, mut other: Box<SkipNode<K, T>>) {
        unsafe {
            let len = self.next.len();
            if level_ >= len {
                return;
            }
            self.next[level_] = Some(Shared::new(Box::into_raw(other)));
            let mut node = self.next[level_];
        } // end unsafe
    }
}



pub struct SkipList<K, T> {

    pub header : NodeBox<K, T>,
    pub level: usize,
    pub prob: f64,
    //marker: PhantomData<Box<SkipNode<K, T>>>

}

impl<K, T> SkipList<K, T>
    where K: Clone + Ord + PartialOrd + Eq + fmt::Display,
          T: Clone{

    pub fn new(level_: usize, prob_: f64) -> Self {

        SkipList {
            header: None,
            level: level_,
            prob: prob_
        }
    }

    pub fn set(&mut self, key_: K, value_: T) {

        unsafe {
            let new_node = SkipList::make_node(key_.clone(), value_.clone());

            let mut level = self.level.clone();
            let mut prev_k = String::from("");

            match self.header {
                None => {
                    self.header = Some(Shared::new(Box::into_raw(new_node)));
                    return;
                },
                Some(ref head) => {
                    let mut cur_node = &self.header;
                    let mut next_node = &self.header;
                    let mut prev_nodes = Vec::new();
                    for i in 0..12 {
                        prev_nodes.push(&self.header);
                    }
                    let mut k = (***head).key.clone();
                    while level > 0 {
                        while k < key_ {
                            //println!("1");
                            if let Some(ref n) = *next_node {
                                //prev_k = k.clone();
                                k = (***n).key.clone();
                                //
                                cur_node = next_node;
                                next_node = &(***n).next[level-1];
                            } else {
                                break;
                            }
                        }
                        prev_nodes[level-1] = cur_node;
                        level -= 1;
                    }
                    // goto last node
                    println!("2");
                    if let Some(ref n) = *next_node {
                        k = (***n).key.clone();
                        if k == key_ {
                            (***n).value = value_.clone();
                            return;
                        } else {

                        }
                    } else {
                        // x->forward is none

                    }
                    let mut lvl = self.random_level();
                    println!("lvl: {}", lvl);
                    let mut l = self.level.clone();
                    if lvl > l {
                        let mut idx = l+1;
                        while idx < lvl {
                            prev_nodes[idx-1] = &self.header;
                            idx += 1;
                        }
                        self.level = lvl;
                    }
                    let mut update_idx = 1;
                    let new_node_ptr = Shared::new(Box::into_raw(new_node));
                    while update_idx <= l {

                        if let Some(mut prev_node) = *prev_nodes[update_idx-1] {
                            (**new_node_ptr).next[update_idx-1] = (**prev_node).next[update_idx-1];
                            //*prev_nodes[update_idx-1] = Some(new_node_ptr);
                            (**prev_node).next[update_idx-1] = Some(new_node_ptr);

                            //prev_node = new_node_ptr;
                        }
                        // test
                        update_idx += 1;
                    }
                }
            }
        }


    }

    pub fn make_node(key_: K, value_: T) -> Box<SkipNode<K, T>> {

        let sk_node = SkipNode::new(key_, value_, 12);
        return Box::new(sk_node);
    }

    fn random_level(&self) -> usize {

        let mut level: usize = 1;
        let mut gen_prob = rand::random::<f64>();
        while gen_prob < self.prob && level < 12 {
            gen_prob = rand::random::<f64>();
            level += 1;
        }
        return level;

    }

    pub fn size(&self) -> usize {

        unsafe {
            let mut len = 0;
            let mut cur_node = &self.header;

            while (*cur_node).is_none() != true {
                if let Some(ref node) = *cur_node {
                    len += 1;
                    cur_node = &(***node).next[0];
                    //cur_node = &((**node).next[0]);
                }
            }


            return len;
        }
    }

    pub fn level_size(&self, level: usize) -> usize {
        unsafe {
            let mut len = 0;
            let mut cur_node = &self.header;

            while (*cur_node).is_none() != true {
                if let Some(ref node) = *cur_node {
                    len += 1;
                    cur_node = &(***node).next[level];
                    //cur_node = &((**node).next[0]);
                }
            }


            return len;
        }
    }

    pub fn print_key_idx(&self, idx: usize){

        unsafe {
            let mut i = 0;
            let mut cur_node = &self.header;
            while (*cur_node).is_none() != true {
                if let Some(ref node) = *cur_node {
                    if i < idx {
                        i += 1;
                        cur_node = &(***node).next[0];
                    } else {
                        let k = (***node).key.clone();
                        println!("idx key: {}", k);
                        break;
                    }
                }
            }
        }


    }
}

/*
The basic node foundation of SkipList
key-value is just k-v for search, get and put
forward_nodes is all the next nodes with different level that it forward
*/
