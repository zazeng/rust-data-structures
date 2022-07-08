#[derive(Debug, PartialEq, PartialOrd)]
struct Node<'a> {
    val: i64,
    next: Option<&'a mut Node<'a>>,
}

struct LinkedList<'a> {
    head: Node<'a>
}

impl<'a>  LinkedList<'a> {
    fn add(&mut self, node: &'a mut Node<'a>) {
        let mut cur_node = &mut self.head;

        loop {
            match cur_node.next {
                Some(ref mut node) => cur_node = node,
                None => {
                    break cur_node.next = Some(node);
                }
            }
        }
    }

    fn add_at(&mut self, node: &'a mut Node<'a>, idx: i64) {
        let mut cur_node = &mut self.head;
        let mut cur_node_idx = 0;

        for i in 0..idx {
            match cur_node.next {
                Some(ref mut node) => {
                    cur_node = node;
                    cur_node_idx = i;
                }
                None => {
                    if idx > i + 1  {
                        panic!("idx {} greater than linkedlist length: {}", idx, i);
                    }
                    cur_node_idx = 1;
                }
            }
        }

        if cur_node_idx > idx {
            cur_node.next = Some(node);
        } else {
            // TODO
        }
    }

    fn peek_last(&self) -> &'a Node {
        let mut cur_node = &self.head;

        loop {
            match cur_node.next {
                Some(ref node) => cur_node = node,
                None => { return cur_node; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init() {
        let linked_list = LinkedList{
            head: Node{ val: 1, next: None }
        };
        assert_eq!(linked_list.head, Node{ val: 1, next: None });
    }

    #[test]
    fn test_peek_last() {
        let linked_list = LinkedList{
            head: Node{ val: 1, next: None }
        };
        assert_eq!(linked_list.peek_last(), &Node{ val: 1, next: None });
    }

    #[test]
    fn test_add() {
        let mut linked_list = LinkedList {
            head: Node{ val: 1, next: None }
        };
        let mut node_2 = Node{ val: 2, next: None };
        linked_list.add(&mut node_2);
        assert_eq!(linked_list.peek_last(), &Node{ val: 2, next: None })
    }
}
