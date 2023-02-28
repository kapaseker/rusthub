use std::collections::HashMap;
use std::process::id;
use strum::EnumCount;
use strum_macros::{Display, EnumCount, FromRepr};
use rand::Rng;

#[derive(Eq, Hash, PartialEq, Clone, Copy, FromRepr, EnumCount, Display, Debug)]
enum ShirtColor {
    Red,
    Blue,
    Green,
    Purple,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let mut color = user_preference.unwrap_or_else(|| self.most_stocked());
        let mut idx = 0;
        for x in self.shirts.iter() {
            if *x == color {
                break;
            }
            idx += 1;
        }
        if idx >= self.shirts.len() {
            color = self.most_stocked()
        }
        let mut idx = 0;
        for x in self.shirts.iter() {
            if *x == color {
                break;
            }
            idx += 1;
        }
        self.shirts.swap_remove(idx);
        println!("removed {}", color);
        return color;
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut colorMap: HashMap<&ShirtColor, i32> = HashMap::new();

        for color in &self.shirts {
            let op = colorMap.get(color);
            let count = op.unwrap_or(&0);
            colorMap.insert(color, count + 1);
        }

        let mut maxCount = 0;
        let mut maxEnum = &ShirtColor::Blue;

        for (k, v) in colorMap {
            if maxCount < v {
                maxCount = v;
                maxEnum = k;
            }
        }

        return *maxEnum;
    }
}

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    let mut inventory = Inventory { shirts: vec![] };

    for x in 0..10 {
        inventory.shirts.push(ShirtColor::from_repr(rng.gen_range(0..ShirtColor::COUNT)).unwrap_or(ShirtColor::Red))
    }

    dbg!("{}", &inventory.shirts);

    println!("{}", inventory.giveaway(None));
    println!("{}", inventory.giveaway(Some(ShirtColor::Red)));
    println!("{}", inventory.giveaway(Some(ShirtColor::Red)));
}