use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    rc::Rc,
};

#[cfg(test)]
mod node_tests;

#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub(crate) is_sentinel: bool,
}

impl<T: Default> Node<T> {
    pub(crate) fn new(key: T) -> Self {
        Self {
            left: None,
            right: None,
            color: NodeColor::Black,
            parent: None,
            key,
            is_sentinel: false,
        }
    }

    pub(crate) fn new_sentinel() -> Self {
        Self {
            left: None,
            right: None,
            color: NodeColor::Black,
            parent: None,
            key: T::default(),
            is_sentinel: true,
        }
    }

    pub(crate) fn set_left_child(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.left = Some(node)
    }

    pub(crate) fn set_right_child(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.right = Some(node)
    }

    pub(crate) fn set_parent(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.parent = Some(node)
    }

    pub(crate) fn parent(&self) -> &Rc<RefCell<Node<T>>> {
        self.parent.as_ref().unwrap()
    }

    pub(crate) fn parent_mut(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.parent.as_mut().unwrap()
    }

    pub(crate) fn left(&self) -> &Rc<RefCell<Node<T>>> {
        self.left.as_ref().unwrap()
    }

    pub(crate) fn left_mut(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.left.as_mut().unwrap()
    }

    pub(crate) fn right(&self) -> &Rc<RefCell<Node<T>>> {
        self.right.as_ref().unwrap()
    }

    pub(crate) fn right_mut(&mut self) -> &mut Rc<RefCell<Node<T>>> {
        self.right.as_mut().unwrap()
    }

    pub(crate) fn is_nil(&self) -> bool {
        self.is_sentinel
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.left.is_some() {
            write!(f, "{:?}", self.left.as_ref().unwrap().borrow())?;
        }

        write!(f, "key: {:?} color: {:?}", self.key, self.color)?;

        if self.right.is_some() {
            write!(f, "{:?}", self.right.as_ref().unwrap().borrow())?;
        }

        Ok(())
    }
}
