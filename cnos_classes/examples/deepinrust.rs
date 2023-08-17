use std::cell::{Cell, UnsafeCell};
use std::mem::MaybeUninit;
use std::ops::Index;
use std::os::unix::raw::blkcnt_t;
use std::primitive;
use std::sync::mpsc::TryIter;
use futures::sink::Buffer;

fn main() {
    {
        block("char array");
        let t_char = b't';
        let hello_arr = b"this is my \n hello";
        let raw_hello_arr = br"this is my \n hello"; // raw string

        println!("{:?}", hello_arr);
        println!("{:?}", raw_hello_arr);
    }

    {
        block("number");
        let a: u8 = 0;
        let better_num = 12_345_678;

        println!("{better_num}");
        println!("9 power 3 = {}", 9_u32.pow(3));
    }

    {
        block("safe add");
        let a: u8 = 250;
        println!("{:?}", a.checked_add(a));
        println!("{:?}", a.saturating_add(a));
        println!("{:?}", a.wrapping_add(a));
    }

    {
        block("default struct");

        struct Point {
            x: i32,
            y: i32,
        }

        impl Default for Point {
            fn default() -> Self {
                Point { x: 0, y: 0 }
            }
        }

        let pt: Point = Point::default();
        let pt = Point { ..Point::default() };
        let Point { x: px, y: py } = pt;
        println!("{},{}", px, py);
    }

    {
        block("enum");

        enum Number {
            I32(i32),
            F32(f32),
        }

        let a = Number::I32(48);

        fn make_match(some: &Number) {
            match some {
                Number::I32(i) => {
                    println!("this is an int {}", i);
                }
                Number::F32(f) => {
                    println!("this is a float {}", f);
                }
            }
        }

        make_match(&a);
        make_match(&Number::F32(32.0));
    }

    {
        block("表达式与语句");
        let a: u8 = 0b_1110_111;
        println!("{:08b}", !a);
        println!("{:08b}", a & 0);

        let x = println!("good");
        let y = {
            println!("good");
            5
        };

        println!("{:?}", x);
        println!("{:?}", y)
    }

    {
        block("function");

        fn add_one(t: (i32, i32)) -> i32 {
            return t.0.saturating_add(t.1);
        }

        fn add_two((x, y): (i32, i32)) -> i32 {
            x.saturating_add(y)
        }

        let mut fc: fn((i32, i32)) -> i32 = add_one;
        println!("{}", fc((32, 45)));
        fc = add_two;
        println!("{}", fc((32, 45)));
    }

    {
        block("trait");

        trait Company {
            fn work(&self);
        }

        trait Home {
            fn work(&self);
        }

        struct Person();

        impl Home for Person {
            fn work(&self) {
                println!("Home working !");
            }
        }

        impl Company for Person {
            fn work(&self) {
                println!("Company work for salary !");
            }
        }

        let teacher = Person();
        Company::work(&teacher);
        Home::work(&teacher);

        println!("min of nan and 3.0 : {}", 3.0f64.min(f64::NAN));
        println!("max of nan and 3.0 : {}", 3.0f64.max(f64::NAN));
    }

    {
        block("array");
        let v = [1, 2, 3, 4];
        let v_2 = [1, 2, 3, 5];
        println!("{:?}", v < v_2);

        struct Rectangle(i32, i32, i32, i32);
        impl Index<u32> for Rectangle {
            type Output = i32;

            fn index(&self, index: u32) -> &Self::Output {
                return match index % 4 {
                    0 => &self.0,
                    1 => &self.1,
                    2 => &self.2,
                    _ => &self.3
                };
            }
        }

        let r = Rectangle(2, 3, 4, 5);
        println!("{}", r[0]);
        println!("{}", r[5]);
    }

    {
        block("pattern destruction");
        let x = (1, 2, 3, 4);

        let (first, ..) = x;
        println!("{}", first);
    }

    {
        block("inner variant");
        let data: Cell<i32> = Cell::new(32);
        data.set(100);
        println!("{}", data.get());
        data.set(980);
        println!("{}", data.get());
    }

    {
        block("unsafe");
        let x = 1_i32;
        let mut y: u32 = 1;
        let raw_y = &mut y as *mut u32 as *mut i32;
        unsafe {
            *raw_y = -1;
        }

        println!("x is :{:X}, y is :{:X}", x, y);

        let vec = vec![1, 2, 3, 4, 4, 5, 6, 7, 8];
        unsafe {
            let t: (usize, usize, usize) = std::mem::transmute_copy(&vec);
            println!("{}, {}, {}", t.0, t.1, t.2)
        }

        let aptr = 30;
        let mut bptr = 23;

        unsafe {
            std::ptr::write(&mut bptr, aptr);

            let a: u32 = MaybeUninit::uninit().assume_init();
            println!("uninitialized value :{}", std::ptr::read(&a));
        }

        println!("{}", bptr);
    }

    {
        block("split array");

        let mut arr = [1, 2, 3, 4, 5];
        let (head, tail) = arr.split_at_mut(2);
        head[0] = 23;
        tail[0] = 100;
        println!("{:?}", arr);
    }

    {
        block("vec");

        let mut v1 = Vec::<i32>::new();
        println!("Start: length {} capacity {}", v1.len(), v1.capacity());

        for i in 1..10 {
            v1.push(i);
            println!("[Pushed {}] length {} capacity {}", i, v1.len(), v1.capacity());
        }

        let mut v2 = Vec::<i32>::with_capacity(1);
        println!("Start: length {} capacity {}", v2.len(), v2.capacity());
        v2.push(10);
        v2.reserve(20);
        for i in 1..10 {
            v2.push(i);
            println!("[Pushed {}] length {} capacity {}", i, v2.len(), v2.capacity());
        }
    }

    {
        block("closure");
    }

    {
        trait Draw {
            fn draw(&self);
        }

        struct Button {}

        struct Text {}

        impl Draw for Button {
            fn draw(&self) {
                println!("Draw button");
            }
        }

        impl Draw for Text {
            fn draw(&self) {
                println!("Draw text");
            }
        }

        fn useDrawBox(item: &dyn Draw) {
            item.draw()
        }

        fn use_draw_boxs(items: &Vec<&dyn Draw>) {
            println!("batch draw !!!");
            for i in items {
                i.draw()
            }
        }

        // fn use_draw_boxs(items: &Vec<&impl Draw>) {
        //     println!("batch draw !!!");
        //     for i in items {
        //         i.draw()
        //     }
        // }
        // `impl Trait` only allowed in function and inherent method return types, not in variable bindings

        // fn useDrawBox(item: &impl Draw) {
        //     item.draw()
        // }

        useDrawBox(&(Button {}));
        useDrawBox(&(Text {}));

        let vec: Vec<&dyn Draw> = vec![&Button {}, &Text {}, &Button {}];
        use_draw_boxs(&vec);
    }
}

fn block(msg: &str) {
    println!("----------------------{}-------------------------", msg);
}