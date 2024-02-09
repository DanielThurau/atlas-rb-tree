use crate::node::{Node, NodeColor};

#[test]
fn test_node_new() {
    let key = 10;
    let node = Node::new(key);

    assert_eq!(node.key, key);
    assert_eq!(node.color, NodeColor::Black);
    assert!(node.left.is_none());
    assert!(node.right.is_none());
    assert!(node.parent.is_none());
}