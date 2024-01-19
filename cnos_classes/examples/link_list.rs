use crate::Node::{NULL, VAL};

enum Node {
    VAL(i32, Box<Node>),
    NULL,
}

impl Node {
    fn build(node: &[i32]) -> Self {
        let mut head = NULL;

        let mut i = node.len();

        loop {
            if i > 0 {
                head = VAL(node[i - 1], Box::new(head));
                i -= 1;
            } else {
                break head;
            }
        }
    }

    fn print(&self) {
        let mut head = self;
        loop {
            if let VAL(value, next) = head {
                print!("{}->", value);
                head = next
            } else {
                println!("null");
                break;
            }
        }
    }
}


fn main() {
    let head = Node::build(&[0, 1, 2, 3, 4, 5, 6]);
    head.print();

    let head = Node::build(&[1, 2, 3, 4, 5, 6, 7]);
    head.print()
}