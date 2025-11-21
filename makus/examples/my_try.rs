use anyhow::Result;


/// simulate ? for my_try
#[macro_export]
macro_rules! my_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }
    };
}
fn main() -> Result<()> {
    let one = my_try!("1".parse::<i32>());
    println!("{}", one);
    let err = my_try!("2".parse::<i32>());
    println!("{}", err);
    Ok(())
}
