fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

// fn duplicate(a: i32) -> (i32, i32) {
//     (a.clone(), a.clone())
// }
//
// fn duplicate(a: f64) -> (f64, f64) {
//     (a.clone(), a.clone())
// }

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn min<T: Ord>(a: T, b: T) -> T {
    if a > b {
        b
    } else {
        a
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int);
    dbg!(from_bool);

    let debuger = pair_of(32);
    
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
