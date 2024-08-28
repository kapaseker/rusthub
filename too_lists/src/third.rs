use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

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

    pub fn prepend(&self, elem: T) -> List<T> {
        Self {
            head: Some(
                Rc::new(
                    Node {
                        elem,
                        next: self.head.clone(),
                    }
                )
            )
        }
    }

    pub fn tail(&self) -> List<T> {
        Self {
            head: self.head.as_deref().and_then(|s| s.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|s| &s.elem)
    }

    // pub fn push(&mut self, elem: T) {
    //     let node = Box::new(Node {
    //         elem,
    //         next: self.head.take(),
    //     });
    //     self.head = Some(node)
    // }
    //
    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|s| {
    //         self.head = s.next;
    //         s.elem
    //     })
    // }
    //
    // pub fn peek(&self) -> Option<&T> {
    //     self.head.as_ref().map(|s| &s.elem)
    // }
    //
    // pub fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.head.as_mut().map(|s| &mut s.elem)
    // }
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

/// Drop implementation
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_head = self.head.take();
        while let Some(node) = cur_head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur_head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;
    use crate::third::List;

    #[test]
    fn case_prepend() {
        let l = List::new();
        let l = l.prepend(3);
        let l = l.prepend(2);

        assert_eq!(l.head(), Some(&2));
    }

    #[test]
    fn case_iter() {
        let l = List::new();
        let l = l.prepend(3);
        let l = l.prepend(2);
        let l = l.prepend(1);

        let mut output = String::new();
        for x in l.iter() {
            output.push_str(&format!("{}", x));
        }

        assert_eq!(output, "123");
    }

    #[test]
    fn case_rc() {
        let l = Rc::new(1);
        let a = l.clone();
        {
            let b = l.clone();
            println!("{}", Rc::strong_count(&l));
        }

        println!("{}", Rc::strong_count(&l));
    }

    #[test]
    fn case_box() {
        let mut a = Box::new(32);
        *a += 90;
        println!("{}", *a);
    }

    #[test]
    fn case_rc_refcell_mut() {
        let mut a = Rc::new(RefCell::new(32));
        let mut b = a.borrow_mut();
        *b = 32;
        println!("{}", b.deref());
    }

    #[test]
    fn case_rc_mut() {

        #[derive(Clone)]
        struct Foo(i32);
        let mut a = Rc::new(Foo(32));
        let b = Rc::make_mut(&mut a);
        b.0 += 100;

        println!("{}", a.0);
    }
}