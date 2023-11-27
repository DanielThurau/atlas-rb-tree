enum NodeColor {
    Red,
    Black,
}

pub (crate) struct Node<T> {
    pub(crate) left: Box<Option<Node<T>>>,
    pub(crate) right: Box<Option<Node<T>>>,
    pub(crate) color: NodeColor,
    pub(crate) parent: Box<Option<Node<T>>>,
    pub(crate) key: T,
}

impl<T> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            left: Box::new(None),
            right: Box::new(None),
            color: NodeColor::Black,
            parent: Box::new(None),
            key,
        }
    }
}