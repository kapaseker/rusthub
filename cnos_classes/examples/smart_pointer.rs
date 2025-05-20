struct Dog {
    name: String,
    age: i8,
}

struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

fn main() {
    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}