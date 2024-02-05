use crate::node::{Node, NodeColor};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

#[cfg(test)]
mod tree_tests;

pub struct Tree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
}

// TODO cleanup traits. For instance, debug might be to strict.
impl<T: Ord + Clone + PartialEq + Debug> Tree<T> {
    pub fn new(key: T) -> Tree<T> {
        Self::new_from_node(Node::new(key))
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
                    let x = z.borrow().parent_unwrap().borrow().parent_unwrap().clone();
                    self.left_rotate(x);
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
            panic!(
                "Invariant violated. The right child of {:?} must not be None.",
                x
            );
        }
    }

    fn right_rotate(&mut self, y: Rc<RefCell<Node<T>>>) {
        if y.borrow().left.is_some() {
            let x = y.clone().borrow_mut().left.take().unwrap();
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
            panic!(
                "Invariant violated. The left child of {:?} must not be None.",
                y
            );
        }
    }

    pub fn delete(&mut self, key: T) {
        let node = self.search(key);
        if node.is_some() {
            self.delete_node(node.as_ref().unwrap().clone())
        }
    }

    fn delete_node(&mut self, z: Rc<RefCell<Node<T>>>) {
        let mut y = z.clone();
        let mut y_color = y.borrow().color.clone();
        let mut x;
        if z.borrow().left.is_none() {
            x = z.borrow().right.clone();
            let u = z.clone();
            let v = z.borrow().right.clone();
            self.transplant(u, v);
        } else if z.borrow().right.is_none() {
            x = z.borrow().left.clone();
            let u = z.clone();
            let v = z.borrow().left.clone();
            self.transplant(u, v);
        } else {
            y = self
                .minimum_node(z.borrow().right.clone().unwrap())
                .expect("Expected this to be set");
            y_color = y.borrow().color.clone();
            x = y.borrow().right.clone();
            if Some(y.clone()) != z.borrow().right {
                let u = y.clone();
                let v = y.borrow().right.clone();
                self.transplant(u, v);
                y.borrow_mut().right = z.borrow().right.clone();
                y.borrow_mut()
                    .right_mut_unwrap()
                    .borrow_mut()
                    .set_parent(Some(y.clone()));
            } else {
                if x.is_some() {
                    x.as_mut().unwrap().borrow_mut().set_parent(Some(y.clone()));
                }
            }
            let u = z.clone();
            let v = Some(y.clone());
            self.transplant(u, v);
            y.borrow_mut().set_left_child(z.borrow().left.clone());
            y.borrow_mut()
                .left_mut_unwrap()
                .borrow_mut()
                .set_parent(Some(y.clone()));
            y.borrow_mut().color = z.borrow().color.clone();
        }

        if y_color == NodeColor::Black {
            self.delete_fix_up(x.unwrap());
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

    fn delete_fix_up(&mut self, mut x: Rc<RefCell<Node<T>>>) {
        while Some(x.clone()) != self.root && x.borrow().color == NodeColor::Black {
            if Some(x.clone()) == x.borrow().parent.as_ref().unwrap().borrow().left {
                let mut w = x
                    .borrow()
                    .parent_unwrap()
                    .borrow()
                    .right
                    .as_ref()
                    .unwrap()
                    .clone();
                // Case 1
                if w.borrow().color == NodeColor::Red {
                    w.borrow_mut().color = NodeColor::Black;
                    x.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Red;
                    self.left_rotate(x.borrow().parent.as_ref().unwrap().clone());
                    w = x
                        .borrow()
                        .parent_unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap()
                        .clone();
                }

                if w.borrow().left.as_ref().unwrap().borrow().color == NodeColor::Black
                    && w.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black
                {
                    w.borrow_mut().color = NodeColor::Red;
                    let x_tmp = x.borrow().parent_unwrap().clone();
                    x = x_tmp;
                } else {
                    // Case 3
                    if w.borrow_mut().right.as_mut().unwrap().borrow().color == NodeColor::Black {
                        w.borrow_mut().left.as_mut().unwrap().borrow_mut().color = NodeColor::Black;
                        w.borrow_mut().color = NodeColor::Red;
                        self.right_rotate(w.clone());
                        w = x
                            .borrow()
                            .parent_unwrap()
                            .borrow()
                            .right
                            .as_ref()
                            .unwrap()
                            .clone();
                    }
                    // Case 4
                    w.borrow_mut().color = x.borrow().parent_unwrap().borrow().color.clone();
                    x.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    w.borrow_mut().right_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    self.left_rotate(x.borrow().parent_unwrap().clone());
                    x = self.root.as_ref().unwrap().clone();
                }
            } else {
                let mut w = x.borrow().parent_unwrap().borrow().left_unwrap().clone();
                // Case 5
                if w.borrow_mut().color == NodeColor::Red {
                    w.borrow_mut().color = NodeColor::Black;
                    x.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Red;
                    self.right_rotate(x.borrow().parent_unwrap().clone());
                    w = x.borrow().parent_unwrap().borrow().left_unwrap().clone();
                }
                // Case 6
                if w.borrow().right_unwrap().borrow().color == NodeColor::Black
                    && w.borrow().left_unwrap().borrow().color == NodeColor::Black
                {
                    w.borrow_mut().color = NodeColor::Red;
                    let x_tmp = x.borrow().parent_unwrap().clone();
                    x = x_tmp;
                } else {
                    // Case 7
                    if w.borrow().left_unwrap().borrow().color == NodeColor::Black {
                        w.borrow_mut().right_mut_unwrap().borrow_mut().color = NodeColor::Black;
                        w.borrow_mut().color = NodeColor::Red;
                        self.left_rotate(w.clone());
                        w = x
                            .borrow_mut()
                            .parent_unwrap()
                            .borrow()
                            .left_unwrap()
                            .clone();
                    }
                    // Case 8
                    w.borrow_mut().color = x.borrow().parent_unwrap().borrow().color.clone();
                    x.borrow_mut().parent_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    w.borrow_mut().left_mut_unwrap().borrow_mut().color = NodeColor::Black;
                    self.right_rotate(x.borrow().parent_unwrap().clone());
                    x = self.root.as_ref().unwrap().clone();
                }
            }
        }
        x.borrow_mut().color = NodeColor::Black;
    }

    pub fn contains_key(&self, key: T) -> bool {
        self.search(key).is_some()
    }

    fn search(&self, key: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut node = self.root.clone();
        while node.is_some() {
            let node_key = node.as_ref().unwrap().borrow().key.clone();
            if node_key == key {
                return node;
            } else if key < node_key {
                let node_tmp = node.as_ref().unwrap().borrow().left.clone();
                node = node_tmp;
            } else {
                let node_tmp = node.as_ref().unwrap().borrow().right.clone();
                node = node_tmp;
            }
        }
        None
    }

    pub fn minimum(&self) -> Option<T> {
        if self.root.is_none() {
            return None;
        }

        self.minimum_node(self.root.as_ref().unwrap().clone())
            .and_then(|node| Some(node.borrow().key.clone()))
    }
    fn minimum_node(&self, node: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        if node.borrow().left.is_none() {
            return Some(node);
        }

        let mut x = node.clone();
        while x.borrow().left.is_some() {
            let x_tmp = x.borrow().left.as_ref().unwrap().clone();
            x = x_tmp;
        }
        Some(x)
    }

    pub fn maximum(&self) -> Option<T> {
        if self.root.is_none() {
            return None;
        }

        self.maximum_node(self.root.as_ref().unwrap().clone())
            .and_then(|node| Some(node.borrow().key.clone()))
    }
    fn maximum_node(&self, node: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
        if node.borrow().right.is_some() {
            return Some(node);
        }

        let mut x = node.clone();
        while x.borrow().right.is_some() {
            let x_tmp = x.borrow().right.as_ref().unwrap().clone();
            x = x_tmp;
        }

        Some(x)
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
