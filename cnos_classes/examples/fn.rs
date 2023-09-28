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

fn main() {
    apply(|| println!("just once"));
    println!("{}", apply_on_3(|x: u32| x * 32));

    let mut p = Person { age: 12 };
    apply_on_3(|x: u32| {
        p.age += x;
        p.age
    });
    println!("{}", p.age);
}

///
/// # mod foo {
/// pub trait Fn<Args> : FnMut<Args> {
///     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
/// }
///
/// pub trait FnMut<Args> : FnOnce<Args> {
///     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
/// }
///
/// pub trait FnOnce<Args> {
///     type Output;
///
///     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
/// }
/// # }
///