fn display_array<T: std::fmt::Display>(arr: &[T]) {
    for ele in arr {
        print!("{} =>", ele);
    }
    println!("")
}

pub mod vete {
    #[derive(Debug)]
    pub struct Vete {
        pub x: f32,
        pub y: f32,
    }

    impl std::ops::Add for Vete {
        type Output = Vete;

        fn add(self, rhs: Vete) -> Self::Output {
            return Vete {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use crate::display_array;
    use crate::vete;
    use crate::vete::Vete;

    #[test]
    fn test_one() {
        display_array(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_add() {
        let a = Vete { x: 3.5, y: 2.9 };
        let b = Vete { x: 2.89, y: 9.23 };

        let c = a + b;
        dbg!(c);
    }
}
