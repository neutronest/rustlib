extern crate rustlib;
extern crate rand;
use rustlib::skiplist::{SkipNode, SkipList};
use rand::Rng;

/*
#[test]
fn test_skipnode() {

    let k = String::from("apple");
    let k1 = String::from("orange");
    let k2 = String::from("banana");
    let mut sn = SkipNode::new(k, 0.5, 10);
    let mut sn1 = Box::new(SkipNode::new(k1, 0.75, 10));
    let mut sn2 = Box::new(SkipNode::new(k2, 0.75, 10));
    sn.modify_next(0, sn1);

    // print
    println!("sn.key {}", sn.key);
    if let Some(ref node) = sn.next[0] {
        unsafe {
            println!("sn1 test");
            println!("s1.key {}", (***node).key );
        }

    }
    sn.modify_next(0, sn2);
    if let Some(ref node) = sn.next[0] {
        unsafe {
            println!("sn2 test");
            println!("s2.key {}", (***node).key );
        }

    }
}
*/


#[test]
fn test_skiplist() {

    println!("test skiplist");
    /*
    let k = String::from("apple");
    let k1 = String::from("orange");
    let k2 = String::from("banana");
    //let mut sn = SkipNode::new(k.clone(), 0.5, 10);
    //let mut sn1 = Box::new(SkipNode::new(k1.clone(), 0.75, 10));
    //let mut sn2 = Box::new(SkipNode::new(k2.clone, 0.75, 10));

    let mut sl = SkipList::<String, f64>::new(1, 0.5);
    sl.set(k, 0.75);
    println!("len: {}", sl.size());
    sl.print_key_idx(0);
    sl.set(k1, 0.25);
    println!("len: {}", sl.size());
    sl.print_key_idx(0);
    sl.print_key_idx(1);
    */
    let mut rng = rand::thread_rng();
    let mut sl = SkipList::<usize, f64>::new(1, 0.5);
    for i in 0..10000 {
        let u = rng.gen::<usize>();
        sl.set(u, 0.75);
        println!("uu: {}", u.clone());

    }
    println!("len: {}", sl.size());
    for j in 0..10 {
        println!("level size {}: {}", j, sl.level_size(j));
    }
}