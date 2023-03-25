use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node<T: Copy> {
    pub value: T,
    pub next: Option<Weak<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T: Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;

pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        match &mut self.tail.take() {
            //if List has no tail, first push adds a Node to the list, which becomes the head AND the tail
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            // If List DOES have a tail, then we need to:
            // 1. Get the value in List.tail and set to the new Node.prev
            // 2. Take the previous Node in the list and point it's .next as the new node
            // 3. update the tail of the List to the new Node.
            Some(old_tail) => {
                node.prev = Some(Rc::downgrade(&old_tail));
                self.tail = node.into();
                old_tail.borrow_mut().next = self.tail.clone(); //remember, clone on an Rc simply creates a new pointer to the same memory + increases the strong reference count!
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            //if no tail, List is empty, so return None
            None => None,
            //If there is a tail we need to...
            Some(tail) => {
                // Get the current tail as mut...
                let mut tail = tail.borrow_mut();

                match tail.prev.take() {
                    //if we're popping the head node, there is no .prev, so need to take self.head
                    None => {
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                }
                Some(tail.value)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works_builds_list() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}
