use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::ops::Deref;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let mut node = Node::new(elem);
        match self.head.take() {
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(old_head);
                self.head = Some(node);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}


#[cfg(test)]
mod test {
    use crate::fourth::List;

    #[test]
    fn case_basic() {
        let mut l = List::new();
        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        assert_eq!(l.pop_front(), Some(3));
        assert_eq!(l.pop_front(), Some(2));
        assert_eq!(l.pop_front(), Some(1));
        assert_eq!(l.pop_front(), None);
    }
}