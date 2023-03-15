fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    for i in &v {
        print!("{} ", i);
    }

    println!("");

    println!("v capacity:{}, length:{}", v.capacity(), v.len());

    let first = v.get(1).unwrap_or(&-1);
    let def = v.get(100).unwrap_or(&-1);

    println!("frist:{},default:{}", first, def);

    let sliceSome = &v[0..2];

    for i in sliceSome {
        println!("slice: {}",i);
    }

    let list:Vec<i32> = Vec::new();

}
