use std::{rc::Rc, sync::Arc, borrow::Borrow};

/**
 * 共享所有权的指针 Rc<T>, Arc<T>
 * Arc用于多线程
 */

/**
 * 内部可变性的只能指针：Cell<T>,RefCell<T>  
 * Cell<T> 值传递的方式对内部变量进行操作
 * RefCell<T> 可获得内部变量的可变引用
 */

fn main() {
    testRc();
    testCell();
    testRefCell();
}

fn testRc() {
    use std::rc::Rc;
    struct Blanket {
        ball: Rc<u8>,
    }
    let rc = Rc::new(1_u8);
    let a = Blanket { ball: rc.clone() };
    let b = Blanket { ball: rc.clone() };

    println!("a is {}", a.ball);
    println!("b is {}", b.ball);
}

fn testCell() {
    use std::cell::Cell;

    let dt = Cell::new(1_u32);

    let refA = &dt;
    let refB = &dt;

    refA.set(100);
    refB.set(200);

    println!("{}", dt.get());
}

fn testRefCell() {
    use std::cell::RefCell;

    struct Apple {
        core: RefCell<u32>,
    }

    impl Apple {
        fn add(&self) {
            let mut f = self.core.borrow_mut();
            *f += 1;
        }
    }

    let apple = Apple {
        core: RefCell::new(23),
    };

    let mut coreMut = apple.core.borrow_mut();

    *coreMut = 26;

    drop(coreMut);

    println!("{}", apple.core.borrow());

    apple.add();

    apple.add();

    println!("{}", apple.core.borrow());

}
