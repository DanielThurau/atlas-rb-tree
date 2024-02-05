use crate::node::{Node, NodeColor};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

#[cfg(test)]
mod tree_tests;

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

pub struct Tree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

// TODO cleanup traits. For instance, debug might be to strict.
impl<T: Ord + Clone + PartialEq +Debug> Tree<T> {
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

    fn empty() -> Tree<T> {
        Tree {
            root: None,
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

        // TODO the constructor guarantees this. It could be removed.
        z.borrow_mut().left = None;
        z.borrow_mut().right = None;
        z.borrow_mut().color = NodeColor::Red;
        self.insert_fix_up(z);
    }

    fn insert_fix_up(&mut self, mut z: Rc<RefCell<Node<T>>>) {
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
                let mut y = z
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .right
                    .clone();
                // Case 1
                if y.is_some() && y.as_ref().unwrap().borrow().color == NodeColor::Red {
                    // println!("Case 1");
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    y.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
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
                    if Some(z.clone()) == z.borrow().parent_unwrap().borrow().right {
                        // println!("Case 2");
                        let z_tmp = z.borrow().parent_unwrap().clone();
                        z = z_tmp;
                        self.left_rotate(z.clone());
                    }
                    // Case 3
                    // println!("Case 3");
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let x = z.borrow().parent_unwrap().borrow().parent_unwrap().clone();
                    self.right_rotate(x);
                }
            } else {
                let mut y = z
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .left
                    .clone();
                // Case 4
                if y.is_some() && y.as_ref().unwrap().borrow().color == NodeColor::Red {
                    // println!("Case 4");
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    y.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
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
                    if Some(z.clone()) == z.borrow().parent_unwrap().borrow().left {
                        // println!("Case 5");
                        let z_tmp = z.borrow().parent_unwrap().clone();
                        z = z_tmp;
                        self.right_rotate(z.clone());
                    }
                    // println!("5={:?}", self.root);
                    // Case 6
                    // println!("Case 6");
                    z.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    z.borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .parent_mut_unwrap()
                        .borrow_mut()
                        .color = NodeColor::Red;
                    let x = z.borrow().parent_unwrap().borrow().parent_unwrap().clone();
                    self.left_rotate(x);
                    // println!("6={:?}", self.root);
                }
            }
        }
        self.root.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
    }

    fn left_rotate(&mut self, x: Rc<RefCell<Node<T>>>) {
        if x.borrow().right.is_some() {
            let y = x.borrow_mut().right.take().unwrap();
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
            let mut x = y.clone().borrow_mut().left.take().unwrap();
            y.borrow_mut().set_left_child(x.borrow().right.clone());
            if x.borrow().right.is_some() {
                x.borrow_mut().right_mut_unwrap().borrow_mut().parent = Some(y.clone());
            }
            x.borrow_mut().set_parent(y.borrow().parent.clone());
            if y.borrow().parent.is_none() {
                self.root = Some(x.clone());
            } else if Some(y.clone()) == y.borrow().parent_unwrap().borrow().right {
                y.borrow_mut().parent_mut_unwrap().borrow_mut().right = Some(x.clone());
            } else {
                y.borrow_mut().parent_mut_unwrap().borrow_mut().left = Some(x.clone());
            }
            x.borrow_mut().right = Some(y.clone());
            y.borrow_mut().parent = Some(x.clone());
        } else {
            panic!("I Don't have the implementation for this yet.")
        }
    }
    fn delete(&mut self, z: Rc<RefCell<Node<T>>>) {
        let y = z.clone();
        let _y_color = &y.borrow().color;
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
        self.minimum_node()
            .and_then(|node| Some(node.borrow().key.clone()))
    }
    fn minimum_node(&self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.root.is_none() {
            return None;
        }

        let mut x = self.root.clone().unwrap();
        while x.borrow().left.is_some() {
            let x_tmp = x.borrow().left.as_ref().unwrap().clone();
            x = x_tmp;
        }
        Some(x)
    }

    pub fn maximum(&self) -> Option<T> {
        self.maximum_node()
            .and_then(|node| Some(node.borrow().key.clone()))
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

/// A DFS implementation using recursion that iterates the
/// entire tree for equality. There are a few speedups I've included,
/// like eliminating base cases and greedily failing.
fn tree_equality_dfs<T: PartialEq>(
    me: Option<Rc<RefCell<Node<T>>>>,
    other: Option<Rc<RefCell<Node<T>>>>,
) -> bool {
    // Solve base cases
    match (me.clone(), other.clone()) {
        (None, None) => return true,
        (Some(_), Some(_)) => (),
        _ => return false,
    }

    if me.as_ref().unwrap().borrow().color != other.as_ref().unwrap().borrow().color {
        return false;
    }

    if me.as_ref().unwrap().borrow().key != other.as_ref().unwrap().borrow().key {
        return false;
    }

    let left_subtree = tree_equality_dfs(
        me.as_ref().unwrap().borrow().left.clone(),
        other.as_ref().unwrap().borrow().left.clone(),
    );
    if !left_subtree {
        return false;
    }

    let right_subtree = tree_equality_dfs(
        me.as_ref().unwrap().borrow().right.clone(),
        other.as_ref().unwrap().borrow().right.clone(),
    );
    if !right_subtree {
        return false;
    }

    true
}

impl<T: PartialEq> PartialEq<Self> for Tree<T> {
    fn eq(&self, other: &Self) -> bool {
        tree_equality_dfs(self.root.clone(), other.root.clone())
    }
}
