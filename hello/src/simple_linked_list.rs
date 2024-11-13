pub struct SimpleLinkedList<T> {
    head: Link<T>,
    size: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    ele: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            ele: _element,
            next: self.head.take(),
        }));

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {

        if self.head.is_some() {
            self.size -= 1;
        }

        self.head.take().map(|s| {
            self.head = s.next;
            s.ele
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|s|{
            &s.ele
        })
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut stack = SimpleLinkedList::new();
        while let Some(v) =  self.pop() {
            stack.push(v)
        }
        stack
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(_iter: I) -> Self {
        let mut stack = Self::new();
        for x in _iter {
            stack.push(x)
        }
        stack
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = vec![];
        let mut _linked_list = _linked_list.rev();
        while let Some(v) =  _linked_list.pop() {
            vec.push(v);
        }
        vec
    }
}

#[cfg(test)]
mod test {
    use crate::simple_linked_list::SimpleLinkedList;

    #[test]

    fn new_list_is_empty() {
        let list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        assert_eq!(list.len(), 0, "list's length must be 0");
    }


    #[test]

    fn push_increments_length() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        list.push(1);


        assert_eq!(list.len(), 1, "list's length must be 1");


        list.push(2);


        assert_eq!(list.len(), 2, "list's length must be 2");
    }


    #[test]

    fn pop_decrements_length() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        list.push(1);


        list.push(2);


        list.pop();


        assert_eq!(list.len(), 1, "list's length must be 1");


        list.pop();


        assert_eq!(list.len(), 0, "list's length must be 0");
    }


    #[test]

    fn is_empty() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        assert!(list.is_empty(), "List wasn't empty on creation");


        for inserts in 0..100 {
            for i in 0..inserts {
                list.push(i);


                assert!(
                    !list.is_empty(),
                    "List was empty after having inserted {i}/{inserts} elements"
                );
            }


            for i in 0..inserts {
                assert!(
                    !list.is_empty(),
                    "List was empty before removing {i}/{inserts} elements"
                );


                list.pop();
            }


            assert!(
                list.is_empty(),
                "List wasn't empty after having removed {inserts} elements"
            );
        }
    }


    #[test]

    fn pop_returns_head_element_and_removes_it() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        list.push(1);


        list.push(2);


        assert_eq!(list.pop(), Some(2), "Element must be 2");


        assert_eq!(list.pop(), Some(1), "Element must be 1");


        assert_eq!(list.pop(), None, "No element should be contained in list");
    }


    #[test]

    fn peek_returns_reference_to_head_element_but_does_not_remove_it() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        assert_eq!(list.peek(), None, "No element should be contained in list");


        list.push(2);


        assert_eq!(list.peek(), Some(&2), "Element must be 2");


        assert_eq!(list.peek(), Some(&2), "Element must be still 2");


        list.push(3);


        assert_eq!(list.peek(), Some(&3), "Head element is now 3");


        assert_eq!(list.pop(), Some(3), "Element must be 3");


        assert_eq!(list.peek(), Some(&2), "Head element is now 2");


        assert_eq!(list.pop(), Some(2), "Element must be 2");


        assert_eq!(list.peek(), None, "No element should be contained in list");
    }


    #[test]

    fn from_slice() {
        let mut array = vec!["1", "2", "3", "4"];


        let mut list: SimpleLinkedList<_> = array.drain(..).collect();


        assert_eq!(list.pop(), Some("4"));


        assert_eq!(list.pop(), Some("3"));


        assert_eq!(list.pop(), Some("2"));


        assert_eq!(list.pop(), Some("1"));
    }


    #[test]

    fn reverse() {
        let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();


        list.push(1);


        list.push(2);


        list.push(3);


        let mut rev_list = list.rev();


        assert_eq!(rev_list.pop(), Some(1));


        assert_eq!(rev_list.pop(), Some(2));


        assert_eq!(rev_list.pop(), Some(3));


        assert_eq!(rev_list.pop(), None);
    }


    #[test]

    fn into_vector() {
        let mut v = Vec::new();


        let mut s = SimpleLinkedList::new();


        for i in 1..4 {
            v.push(i);


            s.push(i);
        }


        let s_as_vec: Vec<i32> = s.into();


        assert_eq!(v, s_as_vec);
    }
}