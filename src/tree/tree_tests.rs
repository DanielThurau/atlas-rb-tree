use crate::{node::Node, tree::Tree};
use proptest::prelude::*;
use std::{cell::RefCell, cmp::min, rc::Rc};

impl<T> Tree<T> {
    fn empty() -> Tree<T> {
        Tree { root: None }
    }
}

#[test]
fn test_left_rotate() {
    // Build a Tree that looks like this:
    //
    //               2
    //             /   \
    //            1      4
    //                  /  \
    //                 3    5
    //
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);
    let mut y = Node::new(4);
    let mut x = Node::new(2);
    y.set_left_child(Some(Rc::new(RefCell::new(b))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(y))));
    let mut actual_tree = Tree::new_from_node(x);

    // Build the expected Tree after performing a left rotate
    //
    //                4
    //               /  \
    //              1    5
    //            /  \
    //           3     2
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);
    let mut y = Node::new(4);
    let mut x = Node::new(2);
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(b))));
    y.set_left_child(Some(Rc::new(RefCell::new(x))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    let expected_tree = Tree::new_from_node(y);

    actual_tree.left_rotate(actual_tree.root.as_ref().unwrap().clone());

    assert_eq!(actual_tree, expected_tree);
}

#[test]
fn test_right_rotate() {
    // Build the Tree after performing a left rotate
    //
    //                4
    //               /  \
    //              1    5
    //            /  \
    //           3     2
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);
    let mut y = Node::new(4);
    let mut x = Node::new(2);
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(b))));
    y.set_left_child(Some(Rc::new(RefCell::new(x))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    let mut actual_tree = Tree::new_from_node(y);

    // Build an expected Tree that looks like this:
    //
    //               2
    //             /   \
    //            1      4
    //                  /  \
    //                 3    5
    //
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);
    let mut y = Node::new(4);
    let mut x = Node::new(2);
    y.set_left_child(Some(Rc::new(RefCell::new(b))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(y))));
    let expected_tree = Tree::new_from_node(x);

    actual_tree.right_rotate(actual_tree.root.as_ref().unwrap().clone());

    assert_eq!(actual_tree, expected_tree);
}

#[test]
fn test_insert() {
    let mut tree = Tree::empty();
    tree.insert(0);
    println!("{:?}", tree);
    tree.insert(1);
    println!("{:?}", tree);
}

#[test]
fn test_delete() {
    let mut tree = Tree::empty();
    tree.insert(0);
    tree.insert(2);
    println!("{:?}", tree);
    println!("{:?}", tree);
    tree.insert(1);
    println!("{:?}", tree);

    tree.delete(2);
    println!("{:?}", tree);
}

proptest! {
    #[test]
    fn test_minimum_empirical(a in 0u32..u32::MAX, b in 0u32..u32::MAX, c in 0u32..u32::MAX) {
        let mut tree = Tree::empty();
        println!("a: {}", a);
        tree.insert(a);
        println!("b: {}", b);
        tree.insert(b);
        println!("c: {}", c);
        tree.insert(c);
        let actual_min = tree.minimum();

        let expected_min = min(min(a, b), c);

        println!("Actual_min={:?}, expected_min={}", actual_min, expected_min);
        prop_assert_eq!(actual_min, Some(expected_min));
    }
}
