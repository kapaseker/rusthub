struct Book {
    title: String,
    author: String,
    year: i32,
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

#[cfg(test)]
mod test {
    use crate::iter::{collection, Book};
    use std::iter::{Filter, Map};
    use std::ops::Div;
    use std::slice::Iter;

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
}
