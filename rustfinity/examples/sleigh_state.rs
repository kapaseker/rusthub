use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// TODO: Define the `status` method for all states

pub struct Sleigh<T:Stateful> {
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

pub trait Stateful {
    fn status() -> &'static str;
}

impl Stateful for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}

impl Stateful for Ready {
    fn status() -> &'static str {
        "Ready"
    }
}

impl Stateful for Flying {
    fn status() -> &'static str {
        "Flying"
    }
}

impl<T: Stateful> Sleigh<T> {
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}
