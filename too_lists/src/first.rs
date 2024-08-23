use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {

    pub fn new() -> Self {
        Self {
            head: Link::Empty
        }
    }

    pub fn push(&mut self, elem: i32) {

        let node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node)

    }

    pub fn pop(&mut self) -> Option<i32> {

        match mem::replace(&mut self.head, Link::Empty) {

            Link::Empty => {
                None
            }

            Link::More(n) => {
                self.head = n.next;
                let pop = n.elem;
                Some(pop)
            }

        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_head = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut n) = cur_head {
            cur_head = mem::replace(&mut n.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn case_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}