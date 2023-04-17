use futures::executor::block_on;

async fn hello_asyn() {
    println!("this is a hello from async fun");
}

async fn get_sum(a:i32,b:i32) -> i32 {
    return  a + b;
}

fn main () {
    block_on(hello_asyn());
    let result = block_on(get_sum(1, 23));
    println!("{}",result);
}