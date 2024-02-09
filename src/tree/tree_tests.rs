use crate::{node::Node, tree::Tree};
use proptest::prelude::*;
use std::{cell::RefCell, cmp::min, rc::Rc};
use crate::node::NodeColor;

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

// TODO tests
// Tests properies of the red black tree. (these are the properties of the red black tree that are abstracted away)
// Test over multiple interesting types (u32, u64, i32, i64, string)
// Test over multiple deleteing types
// Test all public methods

fn is_red<T>(node: Option<Rc<RefCell<Node<T>>>>) -> bool {
    match node {
        Some(node) => node.borrow().color == NodeColor::Red,
        None => false,
    }
}

fn is_black<T>(node: Option<Rc<RefCell<Node<T>>>>) -> bool {
    !is_red(node)
}

fn check_red_node_property<T>(node: Option<Rc<RefCell<Node<T>>>>) -> bool {
    match node {
        Some(n) => {
            if is_red(Some(n.clone())) {
                // Ensure children are black
                is_black(n.borrow().left.clone()) && is_black(n.borrow().right.clone())
            } else {
                // Recursively check children
                check_red_node_property(n.borrow().left.clone()) && check_red_node_property(n.borrow().right.clone())
            }
        },
        None => true, // Nil nodes are black
    }
}

fn count_black_nodes<T>(node: Option<Rc<RefCell<Node<T>>>>) -> Vec<i32> {
    match node {
        Some(n) => {
            let left_counts = count_black_nodes(n.borrow().left.clone());
            let right_counts = count_black_nodes(n.borrow().right.clone());

            // Combine and adjust for current node
            let mut counts = vec![];
            for count in left_counts.into_iter().chain(right_counts.into_iter()) {
                counts.push(count + if n.borrow().color == NodeColor::Black { 1 } else { 0 });
            }
            counts
        },
        None => vec![1], // Nil nodes are black, count as one black node
    }
}


fn assert_red_black_tree_properties<T>(tree: &Tree<T>) {
    if tree.root.is_none() {
        panic!("Assertions on empty red-black trees cause a panic for your own sake")
    }
    // Root property
    assert_eq!(tree.root.as_ref().unwrap().borrow().color, NodeColor::Black);

    // Red node property
    assert!(check_red_node_property(tree.root.clone()));

    // Black height property
    let black_node_counts = count_black_nodes(tree.root.clone());
    // Ensure all paths have the same number of black nodes
    assert!(black_node_counts.iter().min() == black_node_counts.iter().max());
}

#[test]
fn test_insert_maintains_properties() {
    let mut tree = Tree::new(10);
    assert_red_black_tree_properties(&tree);
    tree.insert(5);
    assert_red_black_tree_properties(&tree);
    tree.insert(15);
    assert_red_black_tree_properties(&tree);
    tree.insert(2);
    assert_red_black_tree_properties(&tree);
    tree.insert(7);
    assert_red_black_tree_properties(&tree);
    tree.insert(12);
    assert_red_black_tree_properties(&tree);
    tree.insert(17);
    assert_red_black_tree_properties(&tree);
}

#[test]
fn test_insert_types() {
    let mut string_tree = Tree::new("Red".to_string());
    string_tree.insert("Black".to_string());
    string_tree.insert("Tree".to_string());

    assert_red_black_tree_properties(&string_tree);

    let mut i64_tree = Tree::new(-10_i64);
    i64_tree.insert(0_i64);
    i64_tree.insert(100_i64);

    assert_red_black_tree_properties(&i64_tree);

    let mut f64_tree = Tree::new(-100.03_f64);
    f64_tree.insert(0.0_f64);
    f64_tree.insert(75_f64);

    assert_red_black_tree_properties(&f64_tree);
}

#[test]
fn test_delete_maintains_properties() {
    let mut tree = Tree::new(10);
    tree.delete(10);
    assert!(tree.root.is_none());

    tree.insert(5);
    tree.insert(15);
    tree.insert(2);
    tree.insert(7);
    tree.insert(12);
    tree.insert(17);

    tree.delete(17);
    assert_red_black_tree_properties(&tree);
    tree.delete(7);
    assert_red_black_tree_properties(&tree);
    tree.delete(2);
    assert_red_black_tree_properties(&tree);
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
