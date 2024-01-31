use std::cell::RefCell;
use std::rc::Rc;
use crate::node::Node;
use crate::tree::Tree;

#[test]
fn test_rotate() {
    let a = Node::new(1);
    let b = Node::new(3);
    let c = Node::new(5);

    let mut y = Node::new(4);
    let mut x = Node::new(2);
    y.set_left_child(Some(Rc::new(RefCell::new(b))));
    y.set_right_child(Some(Rc::new(RefCell::new(c))));
    x.set_left_child(Some(Rc::new(RefCell::new(a))));
    x.set_right_child(Some(Rc::new(RefCell::new(y))));

    println!("{:?}", x);
    println!("Root: {:?}", x.key);

    let mut t = Tree::new_from_node(x);

    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);

    t.left_rotate(t.root.as_ref().unwrap().clone());

    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);

    t.right_rotate(t.root.as_ref().unwrap().clone());
    println!("{:?}", t);
    println!("Root: {:?}", t.root.as_ref().unwrap().borrow().key);
}

#[test]
fn test_insert() {
    let mut t = Tree::new(2);
    println!("{:?}", t);

    t.insert(1);
    println!("{:?}", t);

    t.insert(4);
    println!("{:?}", t);

    t.insert(3);
    println!("{:?}", t);
    t.insert(5);
    println!("{:?}", t);
}
