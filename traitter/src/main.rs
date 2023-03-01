use std::fmt::Display;

pub trait Draw {
    fn draw(&self);
}

struct Text {
    label: &'static str,
}

struct Button {
    x: u32,
    y: u32,
    text: &'static str,
}

pub struct Screen<S:Display> {
    controls: Vec<Box<dyn LabelDraw<LB = S>>>,
}

impl Draw for Text {
    fn draw(&self) {
        println!("Text: {}", self.label);
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button {} at [{},{}]", self.text, self.x, self.y);
    }
}

pub trait Label {
    type LB:Display;
    fn get_label(&self) -> Self::LB;
}

impl Label for Button {
    type LB = u32;

    fn get_label(&self) -> Self::LB {
        return self.y;
    }
}

impl Label for Text {
    type LB = &'static str;

    fn get_label(&self) -> Self::LB {
        return self.label;
    }
}

pub trait LabelDraw:Label + Draw {
    
}

impl LabelDraw for Text {
    
}

impl LabelDraw for Button {
    
}

impl <T:Display> Draw for Screen<T> {
    fn draw(&self) {
        for ele in self.controls.iter() {
            print!("element label :{} ",ele.get_label());
            ele.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        controls: vec![
            Box::new(Button {
                x: 100,
                y: 200,
                text: "Good",
            }),
            Box::new(Button {
                x: 200,
                y: 800,
                text: "Job",
            }),
        ],
    };
    screen.draw();
}
