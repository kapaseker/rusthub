fn main() {
    let v_one = vec![1, 2, 3, 4];
    println!("{:?}", v_one);

    let v_two: Vec<i32> = v_one.iter().map(|x| x * 2).collect();
    println!("{:?}", v_one);
    println!("{:?}", v_two);
}