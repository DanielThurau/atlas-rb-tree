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

    pub(crate) fn parent_unwrap(&self) -> &Rc<RefCell<Node<T>>> {
        self.parent.as_ref().unwrap()
    }

    pub(crate) fn parent_mut_unwrap(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.parent.as_mut().unwrap()
    }

    pub(crate) fn left_unwrap(&self) -> &Rc<RefCell<Node<T>>> {
        self.left.as_ref().unwrap()
    }

    pub(crate) fn left_mut_unwrap(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.left.as_mut().unwrap()
    }

    pub(crate) fn right_unwrap(&self) -> &Rc<RefCell<Node<T>>> {
        self.right.as_ref().unwrap()
    }

    pub(crate) fn right_mut_unwrap(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.right.as_mut().unwrap()
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
