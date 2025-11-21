use makus::EnumVar;

#[derive(EnumVar, Debug)]
enum Direction<T, E> {
    Up(DirUp<E>),
    Down(DirDown<T>),
}

#[derive(Debug)]
struct DirUp<E> {
    speed: E,
}

#[derive(Debug)]
struct DirDown<T> {
    gravity: T,
}

fn main() {
    let up: Direction<(), u32> = DirUp { speed: 100 }.into();
    let down: Direction<f32, ()> = DirDown { gravity: 9.8 }.into();

    println!("{:?}", up);
    println!("{:?}", down);
}
