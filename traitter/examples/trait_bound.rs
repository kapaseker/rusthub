trait Bar {
    fn say(&self);
}


struct Apple {
    color: u32,
}

impl Apple {
    fn new() -> Self {
        Apple {
            color: 108,
        }
    }
}

struct Grape {
    alias: String,
}

impl Grape {
    fn new() -> Self {
        Grape {
            alias: "Five Finger".to_string(),
        }
    }
}

impl Bar for Apple {
    fn say(&self) {
        println!("color is :{}", self.color);
    }
}

impl Bar for Grape {
    fn say(&self) {
        println!("grape type is :{}", self.alias)
    }
}

fn sayBar(bar: &impl Bar) {
    bar.say();
}

fn  makeBar() -> impl Bar {
    Grape::new()
}

fn  makeBarDyn() -> Box<dyn Bar> {
    Box::new(Grape::new())
}

fn sayBarRef(bar: &dyn Bar) {
    bar.say()
}

fn main() {
    let apple = Apple::new();
    sayBar(&apple);
    sayBar(&(Grape::new()));

    let bar =  makeBar();
    sayBarRef(&apple);
    sayBarRef(&bar);
}