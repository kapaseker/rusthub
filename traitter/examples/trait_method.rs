struct Button {
    text: String,
    pos: u32,
}

struct Text {
    text: String,
    label: String,
}

pub trait IScreenDisplay {
    fn show(&self);
}

impl IScreenDisplay for Button {
    fn show(&self) {
        println!("I am button on {}", self.pos);
    }
}

impl IScreenDisplay for Text {
    fn show(&self) {
        println!("Text for {} with label {}", self.text, self.label);
    }
}

fn showWidget(widget: &dyn IScreenDisplay) {
    widget.show();
}

fn main() {
    showWidget(&Button {
        text: String::from("Click"),
        pos: 23,
    });
    showWidget(&Text {
        text: String::from("Hello"),
        label: String::from("for China"),
    });
}
