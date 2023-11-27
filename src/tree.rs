use crate::node::{Node}

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
///

struct Tree<T> {
}

impl<T> Tree<T> {
    pub fn insert(&mut self, _key: T) {}

    fn insert_fix_up(&mut self) {}

    pub fn delete(&mut self, _key: T) {}

    fn left_rotate(&mut self, node: Node<T>) {
        // y = x.right               // set y
        // x.right = y.left          // turn y's left subtree into 's right subtree
        // if y.left != None
        //   y.left.p = x
        // y.p = x.p                 // link x's parent to y
        // if x.p == None
        //   T.root  = y
        // elseif x == x.p.left
        //   x.p.left = y
        // else x.p.right = y
        // x.left = x                // put x on y's left
        // x.p = y
    }

    fn right_rote(&mut self, node: Node<T>) {

    }
}