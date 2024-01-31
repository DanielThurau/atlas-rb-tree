use crate::node::{Node, NodeColor};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

/// red-black properties
/// --------------------
///
/// 1. Every node is either red or black
/// 2. The root is black
/// 3. Every leaf (None) is black
/// 4. If a node is red, then both its children are black
/// 5. For each node, all simple paths from the node to
///    descendant leaves contain the same number of black
///    nodes.

struct Tree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Ord + Clone> Tree<T>
where
    Node<T>: PartialEq,
{
    pub fn new(key: T) -> Tree<T> {
        Tree {
            root: Some(Rc::new(RefCell::new(Node::new(key)))),
        }
    }

    fn new_from_node(node: Node<T>) -> Tree<T> {
        Tree {
            root: Some(Rc::new(RefCell::new(node))),
        }
    }

    pub fn insert(&mut self, key: T) {
        let mut z = Node::new(key);
        let mut x = self.root.clone();
        let mut y = None;

        while x.is_some() {
            y = x.clone();
            if z.key < x.as_ref().unwrap().borrow().key {
                let x_tmp = x.as_ref().unwrap().borrow().left.clone();
                x = x_tmp
            } else {
                let x_tmp = x.as_ref().unwrap().borrow().right.clone();
                x = x_tmp;
            }
        }
        z.parent = y.clone();
        // Z is now Reference counted for
        let z = Rc::new(RefCell::new(z));

        if y.is_none() {
            self.root = Some(z.clone());
        } else if z.borrow().key < y.as_ref().unwrap().borrow().key {
            y.as_mut().unwrap().borrow_mut().left = Some(z.clone());
        } else {
            y.as_mut().unwrap().borrow_mut().right = Some(z.clone());
        }
        z.borrow_mut().left = None;
        z.borrow_mut().right = None;
        z.borrow_mut().color = NodeColor::Red;
        self.insert_fix_up(z);
    }

    fn insert_fix_up(&mut self, mut z: Rc<RefCell<Node<T>>>) {
        if z.borrow().parent.is_none() {
            panic!("You violated an invariant. Z's parent cannot be none.");
        }
        while z.borrow().parent.is_some()
            && z.borrow().parent_unwrap().borrow().color == NodeColor::Red
        {
            if z.borrow().parent
                == z.borrow()
                    .parent_unwrap()
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .left
            {
                let y = z
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .right
                    .clone()
                    .unwrap();
                // Case 1
                if y.borrow().color == NodeColor::Red {
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let z_tmp = z.borrow().parent_unwrap().borrow().parent_unwrap().clone();
                    z = z_tmp;
                } else {
                    // Case 2
                    if z == z.borrow().parent_unwrap().borrow().right_unwrap().clone() {
                        let z_tmp = z.borrow().parent_unwrap().clone();
                        z = z_tmp;
                        self.left_rotate(z.clone());
                    }
                    // Case 3
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    self.right_rotate(z.borrow().parent_unwrap().borrow().parent_unwrap().clone());
                }
            } else {
                let y = z
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .left
                    .clone()
                    .unwrap();
                // Case 4
                if y.borrow().color == NodeColor::Red {
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    y.borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let z_tmp = z.borrow().parent_unwrap().borrow().parent_unwrap().clone();
                    z = z_tmp;
                } else {
                    // Case 5
                    if z == z.borrow().parent_unwrap().borrow().left_unwrap().clone() {
                        let z_tmp = z.borrow().parent_unwrap().clone();
                        z = z_tmp;
                        self.right_rotate(z.clone());
                    }
                    // Case 6
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    self.right_rotate(z.borrow().parent_unwrap().borrow().parent_unwrap().clone());
                }
            }
        }
        self.root.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
    }

    fn left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        if x.borrow().right.is_some() {
            let y = x.borrow().right_unwrap().clone();
            x.borrow_mut().set_right_child(y.borrow().left.clone());
            if y.borrow().left.is_some() {
                y.borrow_mut().left_mut_unwrap().borrow_mut().parent = Some(x.clone());
            }
            y.borrow_mut().set_parent(x.borrow().parent.clone());
            if x.borrow().parent.is_none() {
                self.root = Some(y.clone());
            } else if Some(x.clone()) == x.borrow().parent_unwrap().borrow().left {
                x.borrow_mut().parent_mut_unwrap().borrow_mut().left = Some(y.clone());
            } else {
                x.borrow_mut().parent_mut_unwrap().borrow_mut().right = Some(y.clone());
            }
            y.borrow_mut().left = Some(x.clone());
            x.borrow_mut().parent = Some(y);
        } else {
            panic!("I don't have the implementation for this yet.")
        }
    }

    fn right_rotate(&mut self, y: Rc<RefCell<Node<T>>>) {
        if y.borrow().left.is_some() {
            let x = y.borrow_mut().left_mut_unwrap().clone();
            y.borrow_mut().set_left_child(x.borrow().right.clone());
            if x.borrow().right.is_some() {
                x.borrow_mut().right_mut_unwrap().borrow_mut().parent = Some(y.clone());
            }
            x.borrow_mut().set_parent(y.borrow().parent.clone());
            if y.borrow().parent.is_none() {
                self.root = Some(x.clone());
            } else if Some(y.clone()) == y.borrow().parent_unwrap().borrow().left {
                y.borrow_mut().parent_mut_unwrap().borrow_mut().right = Some(x.clone());
            } else {
                y.borrow_mut().parent_mut_unwrap().borrow_mut().left = Some(x.clone());
            }
            x.borrow_mut().right = Some(y.clone());
            y.borrow_mut().parent = Some(x);
        } else {
            panic!("I Don't have the implementation for this yet.")
        }
    }
    pub fn delete(&mut self, z: Rc<RefCell<Node<T>>>) {
        let y = z.clone();
        let y_color = &y.borrow().color;
        let mut x = None;
        if z.borrow().left.is_none() {
            x = z.borrow().right.clone();
            self.transplant(z.clone(), z.borrow().right.clone());
        } else if z.borrow().right.is_none() {
            x = z.borrow().left.clone();
            self.transplant(z.clone(), z.borrow().left.clone());
        } else {
        }
    }

    fn transplant(&mut self, u: Rc<RefCell<Node<T>>>, mut v: Option<Rc<RefCell<Node<T>>>>) {
        if u.borrow().parent.is_none() {
            self.root = v.clone();
        } else if Some(u.clone()) == u.borrow().parent_unwrap().borrow().left {
            u.borrow_mut().parent_mut_unwrap().borrow_mut().left = v.clone();
        } else {
            u.borrow_mut().parent_mut_unwrap().borrow_mut().right = v.clone();
        }

        if v.is_some() {
            v.as_mut().unwrap().borrow_mut().parent = u.borrow().parent.clone();
        }
    }
    pub fn search(&self, _key: T) -> bool {
        todo!()
    }

    pub fn minimum(&self) -> Option<T> {
        self.minimum_node().and_then(|node| Some(node.borrow().key.clone()))
    }
    fn minimum_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        let mut x = self.root.clone();
        while x.is_some() {
            let x_tmp = x.unwrap().borrow().left.clone();
            x = x_tmp;
        }
        x
    }

    pub fn maximum(&self) -> Option<T> {
        self.maximum_node().and_then(|node| Some(node.borrow().key.clone()))
    }
    fn maximum_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        let mut x = self.root.clone();
        while x.is_some() {
            let x_tmp = x.unwrap().borrow().right.clone();
            x = x_tmp;
        }
        x
    }

    pub fn successor(&self) -> T {
        todo!()
    }

    pub fn predecessor(&self) -> T {
        todo!()
    }
}

impl<T: Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

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
