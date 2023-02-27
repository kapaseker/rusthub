use crate::garden::vege::Tomato;

mod garden;

fn main() {
    let moto = Tomato { name: String::from("qiaqia"), weight: 3.45 };
    println!("tomato's name {}", moto.name);
}
