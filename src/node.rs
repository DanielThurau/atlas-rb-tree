use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

#[derive(PartialEq, Eq)]
pub(crate) enum NodeColor {
    Red,
    Black,
}

#[derive(PartialEq, Eq)]
pub(crate) struct Node<T> {
    pub(crate) left: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) right: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) color: NodeColor,
    pub(crate) parent: Option<Rc<RefCell<Node<T>>>>,
    pub(crate) key: T,
}

impl<T> Node<T> {
    pub(crate) fn new(key: T) -> Self {
        Self {
            left: None,
            right: None,
            color: NodeColor::Black,
            parent: None,
            key,
        }
    }

    // TODO Return some kind of indicator if I clobber the left child.
    pub(crate) fn set_left_child(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.left = node
    }

    pub(crate) fn set_right_child(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.right = node
    }

    pub(crate) fn set_parent(&mut self, node: Option<Rc<RefCell<Node<T>>>>) {
        self.parent = node
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.left.is_some() {
            write!(f, "{:?}", self.left.as_ref().unwrap().borrow())?;
        }

        write!(f, "{:?} ", self.key)?;

        if self.right.is_some() {
            write!(f, "{:?}", self.right.as_ref().unwrap().borrow())?;
        }

        Ok(())
    }
}
