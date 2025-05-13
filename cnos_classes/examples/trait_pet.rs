
trait Bird {
    fn fly(&self);
}

trait Car {
    fn wheel(&self) -> i32;
}

struct BirdCar {
    wheel_count: i32,
}

impl Car for BirdCar {
    fn wheel(&self) -> i32 {
        self.wheel_count
    }
}

impl Bird for BirdCar {
    fn fly(&self) {
        println!("fly with {} wheels", self.wheel());
    }
}


trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}

fn main() {
    let puppy = Dog(String::from("Rex"));
    println!("{} has {} legs", puppy.name(), puppy.leg_count());
}