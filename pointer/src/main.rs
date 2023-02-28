use std::ops::Deref;

struct MyPointer<T>(T);



impl<T> MyPointer<T> {
    pub fn new(x:T) -> MyPointer<T> {
        return MyPointer(x);
    }
}

impl<T> Deref for MyPointer<T> {

    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

impl<T> Drop for MyPointer<T>{
    fn drop(&mut self) {
        print!("MyPointer droped");
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    assert_eq!(5, *(MyPointer(5)));
}
