fn main() {
    println!("{}", area((12, 45)));
}

fn area(dimension: (u32, u32)) -> u32 {
    return dimension.0 * dimension.1;
}
