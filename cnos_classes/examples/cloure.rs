use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc_fun: T,
    key: Option<u32>,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(cache_fun: T) -> Self {
        Self {
            calc_fun: cache_fun,
            key: Option::None,
            value: Option::None,
        }
    }

    fn value(&mut self, key: u32) -> u32 {
        match self.key {
            Some(v) => {
                if v == key {
                    return self.value.unwrap_or_default();
                }
            }
            _ => {}
        };
        let cache_value = (self.calc_fun)(key);
        self.key = Option::Some(key);
        self.value = Option::Some(cache_value);
        return cache_value;
    }
}

fn main() {
    let func_closure = |num: i32| -> i32 {
        return num + 23;
    };

    println!("{}", func_closure(100));
    println!("{}", func_closure(9));

    let cacher_fun = |num: u32| -> u32 {
        println!("generate cached value");
        thread::sleep(Duration::from_secs(2));
        num * 23
    };

    let mut cache = Cacher::new(cacher_fun);
    println!("first:{}", cache.value(1));
    println!("second:{}", cache.value(1));
    println!("third:{}", cache.value(2));
}
