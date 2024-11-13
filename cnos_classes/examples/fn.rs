fn apply<F>(f: F)
    where
        F: FnOnce() {
    f();
}

fn apply_on_3<F>(f: F) -> u32 where
    F: FnOnce(u32) -> u32 {
    f(3)
}

struct Person {
    age: u32,
}

struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    apply(|| println!("just once"));
    println!("{}", apply_on_3(|x: u32| x * 32));

    let mut p = Person { age: 12 };
    apply_on_3(|x: u32| {
        p.age += x;
        p.age
    });
    println!("{}", p.age);


    let arrays:Vec<Array<_,3>> = vec![
        Array::<i32, 3>{
            data: [1, 2, 3],
        },
        Array::<i32, 3>{
            data: [1, 2, 3],
        },
        Array::<i32, 3>{
            data: [1, 2, 3],
        },
    ];

    println!("Success!");
}
