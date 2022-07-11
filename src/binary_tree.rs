use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Node<T> {
    pub value: T,
    pub count: i32,
    pub lchild: Option<Box<Node<T>>>,
    pub rchild: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            count: 1,
            lchild: None,
            rchild: None
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct BST<T> {
    root: Option<Node<T>>,
}

impl<T> BST<T> {
    pub fn new(root: Node<T>) -> BST<T> {
        BST { root: Some(root) }
    }

    pub fn root(&self) -> Option<&Node<T>> {
        match self.root {
            Some(ref node) => Some(node),
            None => None,
        }
    }

    pub fn mut_root(&mut self) -> Option<&mut Node<T>> {
        match self.root {
            Some(ref mut node) => Some(node),
            None => None,
        }
    }

    pub fn insert(&mut self, node: Node<T>)
    where
        T: PartialEq + PartialOrd,
    {
        let mut cur_node: &mut Node<T> = self.mut_root().unwrap();

        loop {
            if cur_node.value > node.value {
                match cur_node.lchild {
                    Some(ref mut n) => cur_node = n,
                    None => {
                        cur_node.lchild = Some(Box::new(node));
                        break;
                    }
                }
            } else if cur_node.value < node.value {
                match cur_node.rchild {
                    Some(ref mut n) => cur_node = n,
                    None => {
                        cur_node.rchild = Some(Box::new(node));
                        break;
                    }
                }
            } else {
                cur_node.count += 1;
                break;
            }
        }
    }

    pub fn search(&self, value: T) -> Option<&Node<T>> 
    where
        T: PartialEq + PartialOrd,
    {
        let mut cur_node: & Node<T> = self.root().unwrap();

        loop {
            if cur_node.value < value {
                match &cur_node.rchild {
                    Some(n) => cur_node = n.as_ref(),
                    None => return None
                }
            } else if cur_node.value > value {
                match &cur_node.lchild {
                    Some(n) => cur_node = n.as_ref(),
                    None => return None
                }
            } else {
                return Some(cur_node);
            }
        } 
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let bst = BST::new(Node::new(1));
        assert_eq!(bst.root, Some(Node::new(1)));
    }

    #[test]
    fn root() {
        let bst = BST::new(Node::new(1));
        assert_eq!(bst.root().unwrap(), &Node::new(1));
    }

    #[test]
    fn mut_root() {
        let mut bst = BST::new(Node::new(1));
        assert_eq!(bst.mut_root().unwrap(), &mut Node::new(1));
    }

    #[test]
    fn insert() {
        let expected_bst = BST::new(
            Node {
                value: 1,
                count: 1,
                lchild: None,
                rchild: Some(Box::new(Node {
                    value: 2,
                    count: 2,
                    lchild: None, 
                    rchild: Some(Box::new(Node {
                        value: 4,
                        count: 1,
                        lchild: Some(Box::new(Node {
                            value: 3,
                            count: 1,
                            lchild: None,
                            rchild: None,
                        })),
                        rchild: None,
                    })),
                })),
            });
                    
        let mut bst = BST::new(Node::new(1));
        bst.insert(Node::new(2));
        bst.insert(Node::new(4));
        bst.insert(Node::new(3));
        bst.insert(Node::new(2));

        assert_eq!(bst, expected_bst);
    }

    #[test]
    fn search() {
        let mut bst = BST::new(Node::new(1));
        bst.insert(Node::new(2));

        assert_eq!(bst.search(2), Some(&Node::new(2)));
        assert_eq!(bst.search(3), None);
    }
}
