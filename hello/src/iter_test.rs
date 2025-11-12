#[cfg(test)]
mod test {
    use regex::Regex;
    use std::iter;

    #[test]
    fn test_find_iter() {
        // 查找IP地址
        println!("\n查找IP地址:");
        let ip_re = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
        let ip_text = "服务器地址: 192.168.1.1, 备用地址: 10.0.0.1, 无效IP: 999.999.999.999";

        for mat in ip_re.find_iter(ip_text) {
            println!("找到IP: '{}' at {:?}", mat.as_str(), mat.range());
        }
    }

    #[test]
    fn test_successor() {
        for i in std::iter::successors(
            Some(12345u32),
            |&n| if n == 0 { None } else { Some(n / 10) },
        ) {
            println!("{}", i);
        }
    }

    #[test]
    fn test_chain() {
        let iter1 = vec![1, 2, 3].into_iter();
        let iter2 = vec![4, 5, 6].into_iter();
        let chained: Vec<i32> = iter1.chain(iter2).collect();
        println!("连接: {:?}", chained);

        // 将两个迭代器的元素配对
        let names = vec!["Alice", "Bob", "Charlie"];
        let ages = vec![25, 30, 35];
        let paired: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
        println!("配对: {:?}", paired);

        let single_value = iter::once(42);
        let once_vec: Vec<i32> = single_value.collect();
        println!("once(42) 结果: {:?}", once_vec);

        // 与其它迭代器连接
        let start = iter::once(0);
        let middle = vec![1, 2, 3];
        let end = iter::once(4);
        let combined: Vec<i32> = start.chain(middle).chain(end).collect();
        println!("在 [1,2,3] 前后添加元素: {:?}", combined);

        // 无限重复产生同一个值
        let repeated: Vec<&str> = iter::repeat("hello").take(5).collect();
        println!("重复 'hello' 5次: {:?}", repeated);
    }

    #[test]
    fn test_by_ref() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter();

        // 直接调用 take() 会消耗迭代器
        let first_two: Vec<_> = iter.by_ref().take(2).collect();
        println!("前两个元素: {:?}", first_two);

        // 错误!iter 已经被 take() 消耗了,不能再使用
        let remaining: Vec<_> = iter.collect();
        println!("剩余元素: {:?}", remaining);
    }

    #[test]
    fn test_rev() {
        let chars = vec!['A', 'B', 'C', 'D', 'E'];
        let mut ix = 0;
        chars
            .iter()
            .inspect(|ch| println!("INSPECT: {ch}"))
            .map(|&ch| {
                ix += 1;
                (ix, ch)
            })
            .rev()
            .for_each(|x| println!("PRINT: {x:?}"));
    }
}
