use std::ops::Deref;

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// A base list implementation
impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None
        }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(
            Node {
                elem,
                next: self.head.take(),
            }
        ))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|s| {
            self.head = s.next;
            s.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|s| &s.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|s| &mut s.elem)
    }
}

/// Drop implementation
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_head = self.head.take();
        while let Some(mut n) = cur_head {
            cur_head = n.next.take();
        }
    }
}

pub struct ListIter<T>(List<T>);

/// into_iter() implementation, into_iter() return T
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter(self)
    }
}

impl<T> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

/// iter() implementation, into_iter() return &T
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|v| {
            self.next = v.next.as_deref();
            &(v.elem)
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

/// iter_mut() implementation, into_iter() return &mut T
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|v| {
            self.next = v.next.as_deref_mut();
            &mut (v.elem)
        })
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use crate::second::List;
    use std::mem;

    #[test]
    fn case_push() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn case_peek_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        list.peek_mut().map(|s| { *s = 100 });

        assert_eq!(list.peek(), Some(&100));
    }

    #[test]
    fn case_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut output = String::new();
        for x in list.into_iter() {
            output.push_str(&format!("{}", x))
        }
        assert_eq!("321", output);
    }

    #[test]
    fn case_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut output = String::new();
        for x in list.iter() {
            output.push_str(&format!("{}", x))
        }
        assert_eq!("321", output);
    }

    #[test]
    fn case_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        for x in list.iter_mut() {
            *x = *x * 2;
        }

        let mut output = String::new();
        for x in list.iter() {
            output.push_str(&format!("{}", x))
        }
        assert_eq!("642", output);
    }

    #[test]
    fn case_take() {
        let mut a = 3;
        let b = 4;

        let c = mem::replace(&mut a, b);
        assert_eq!(a, 4);
        assert_eq!(c, 3);
    }

    #[test]
    fn case_box() {

        struct Foo(String);

        let mut a =  Box::new(Foo("abcdef".to_string()));
        // let mut b = a.clone();
        // assert_eq!("abcdef", b.0);
        let c = a.as_mut();
        c.0.push_str("hhh");
        assert_eq!("abcdefhhh", a.0);
    }

    #[test]
    fn case_ref_cell() {
        struct Foo(String);
        let a = RefCell::new(Foo("abcdef".to_string()));
        let mut b = a.borrow_mut();
        b.0.push_str("ccc");

        // let mut c = a.borrow_mut();
        // c.0.push_str("ddd");

        assert_eq!("abcdefcccddd", a.borrow().0);
    }
}