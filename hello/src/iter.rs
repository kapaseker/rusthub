use std::ops::Div;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: i32,
}

impl Book {
    fn century(&self) -> i32 {
        (self.year - 1).div(100) + 1
    }
}

struct BookId(usize);

fn books_with_ids<'a>(
    books_iter: impl Iterator<Item = &'a Book>,
) -> impl Iterator<Item = (BookId, &'a Book)> {
    books_iter.enumerate().map(|(ix, b)| (BookId(ix + 1), b)) // book's id is ix + 1
}

// 模拟获取书籍集合的函数
fn collection() -> Vec<Book> {
    vec![
        Book {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            year: 1949,
        },
        Book {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            year: 1960,
        },
        Book {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            year: 1925,
        },
        Book {
            title: "Pride and Prejudice".to_string(),
            author: "Jane Austen".to_string(),
            year: 1813,
        },
        Book {
            title: "The Catcher in the Rye".to_string(),
            author: "J.D. Salinger".to_string(),
            year: 1951,
        },
        Book {
            title: "Brave New World".to_string(),
            author: "Aldous Huxley".to_string(),
            year: 1932,
        },
        Book {
            title: "One Hundred Years of Solitude".to_string(),
            author: "Gabriel García Márquez".to_string(),
            year: 1967,
        },
    ]
}

struct PipelineBuilder<'a, T> {
    iter: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T: 'a> PipelineBuilder<'a, T> {
    fn new(iter: impl Iterator<Item = T> + 'a) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T: 'a> PipelineBuilder<'a, T> {
    fn with_filter(self, cond: impl FnMut(&T) -> bool + 'a) -> Self {
        Self {
            iter: Box::new(self.iter.filter(cond)),
        }
    }
    fn with_map<S>(self, f: impl FnMut(T) -> S + 'a) -> PipelineBuilder<'a, S> {
        PipelineBuilder {
            iter: Box::new(self.iter.map(f)),
        }
    }
}

impl<'a, T: 'a> PipelineBuilder<'a, T> {
    fn for_each(self, f: impl FnMut(T) -> () + 'a) {
        self.iter.for_each(f)
    }
}

#[cfg(test)]
mod test {
    use crate::iter::{books_with_ids, collection, Book, BookId, PipelineBuilder};
    use rand::random;
    use std::collections::BTreeSet;
    use std::iter::{Filter, Map};
    use std::ops::{Div, Rem};
    use std::slice::Iter;

    #[test]
    fn test_even() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let it: Iter<i32> = numbers.iter();

        let mut pipline = PipelineBuilder::new(it);

        let needs_filtering: bool = random();
        if needs_filtering {
            pipline = pipline.with_filter(|x: &&i32| x.rem(2) == 0);
        }

        pipline.for_each(|x| print!("{} ", x));
    }

    #[test]
    fn test_century() {
        let books = collection();
        books
            .iter()
            .map(Book::century)
            .collect::<BTreeSet<_>>()
            .into_iter() // centuries in the ascending order:
            .for_each(|century| {
                println!("Books from the {} century:", century);
                books
                    .iter()
                    .enumerate()
                    .filter_map(|(ix, b)| {
                        if b.century() == century {
                            Some((ix + 1, b)) // book's id is ix + 1
                        } else {
                            None
                        }
                    })
                    .for_each(|(id, book)| println!("#{id}: {book:?}"));
            });
    }

    #[test]
    fn test_opt_century() {
        let books = collection();
        let centuries: BTreeSet<_> = books.iter().map(Book::century).collect();

        for century in centuries {
            println!("Books from the {} century:", century);
            books_with_ids(books.iter())
                .filter(|(_, b)| b.century() == century)
                .for_each(|(BookId(id), book)| println!("#{id}: {book:?}"));
        }
    }

    #[test]
    fn test_iter() {
        let numbers = vec![10, 15, 20, 25, 30, 35, 40];

        // 使用 enumerate 获取索引和值，
        // filter 保留索引为偶数的项，
        // map 将值翻倍，
        // 最后收集结果为 Vec
        let result: Vec<(usize, i32)> = numbers
            .into_iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0) // 保留偶数索引
            .map(|(i, val)| (i, val * 2)) // 值乘以 2
            .collect();

        println!("{:?}", result);
    }

    #[test]
    fn test_book() {
        // 计算某世纪的书籍数量
        let books: Vec<Book> = collection();

        let number_of_books_from_20 = books
            .iter() // 得到 &Book 的迭代器
            .map(|book| (book.year - 1).div(100) + 1) // 将年份转为世纪：例如 1900 → 19 世纪
            .filter(|&c| c == 20) // 筛选出属于指定世纪的书籍
            .fold(0, |acc, _| acc + 1);
        println!(
            "Number of books from the {} century: {}",
            20, number_of_books_from_20
        );
    }

    #[test]
    fn test_rev() {
        let vec = vec![1, 2, 3, 4, 5];
        let mut iter = vec.iter(); // iter() 返回一个 DoubleEndedIterator

        // 从前往后取一个
        println!("{:?}", iter.next());
        // 从后往前取一个
        println!("{:?}", iter.next_back());
    }

    #[test]
    fn test_len() {
        let data = vec![10, 20, 30, 40];
        let iter = data.iter(); // 实现了 ExactSizeIterator

        println!("总共有 {} 个元素", iter.len()); // 输出：4
    }

    #[test]
    fn test_iter_vec() {
        let mut vec = vec![1, 2, 3, 4, 5];

        // 通过不可变引用遍历 vector，不会获取所有权，vec 在循环后仍可使用
        for i in &vec {
            print!("{} ", i);
        }

        for i in vec.iter() {
            print!("{} ", i);
        }

        println!();

        // 通过可变引用遍历 vector，不获取所有权但可以修改元素，vec 在循环后仍可使用
        for i in &mut vec {
            *i += 1;
            print!("{} ", i);
        }

        for i in vec.iter_mut() {
            print!("{} ", i);
        }

        println!();

        // 通过获取所有权的方式遍历 vector，vec 在循环后不再可用
        for i in vec {
            print!("{} ", i);
        }
    }
}
