#[derive(Debug, PartialEq, PartialOrd)]
pub struct Node {
    val: i64,
    next: Box<Option<Node>>,
}

pub struct LinkedList {
    head: Box<Option<Node>>,
    length: i64,
}

// impl linkedlist {
//     pub fn new(node: node) -> self {
//         self { head: box::new(some(node)), length: 1 }
//     }
//
//     pub fn add(node: node) {
//
//     }
//
//     fn node_at(&self, idx: i64) -> &node {
//         &(*self.head).unwrap()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_init() {
//         let linked_list = linkedlist::new(node{ val: 1, next: box::new(none) });
//         assert_eq!(linked_list.head, some(node{ val: 1, next: box::new(none) }));
//         assert_eq!(linked_list.length, 1);
//     }
// }
