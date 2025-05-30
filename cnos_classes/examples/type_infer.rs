fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    
    
    let a = Some(12);
    
    let Some(b) = a else { 
        println!("a is None");
        return
    };
    
    println!("b = {:?}", b);
}