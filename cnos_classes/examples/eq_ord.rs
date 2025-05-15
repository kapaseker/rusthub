use std::collections::HashMap;

#[derive(PartialEq)]
struct Book {
    name: String,
    price: f32,
}

// impl PartialEq for Book {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.price == other.price
//     }
// }
// 
// impl Eq for Book {}

struct Novel {
    name: String,
    author: String,
}

struct Author {
    name: String,
    age: u32,
}

impl PartialEq<Novel> for Book {
    fn eq(&self, other: &Novel) -> bool {
        other.name == self.name
    }
}

impl PartialEq<Author> for Novel {
    fn eq(&self, other: &Author) -> bool {
        other.name == self.author
    }
}

// impl PartialEq for Book {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.price == other.price
//     }
// }

fn main() {
    let book_harry = Book {
        name: "Harry Potter".to_string(),
        price: 23.6f32,
    };

    let novel_harry = Novel {
        name: "Harry Potter".to_string(),
        author: "JK.Rowling".to_string(),
    };

    let author_jk = Author {
        name: "JK.Rowling".to_string(),
        age: 60,
    };

    assert!(book_harry == novel_harry);
    assert!(novel_harry == author_jk);

    // println!("{}", 0 / 0 == 0 / 0);
    println!("{}", f64::atanh(1.00001));
    
    // let map = HashMap::from([(45f32, "book"), (12f32, "apple")]);
    // 
    // f32::NAN;
}
