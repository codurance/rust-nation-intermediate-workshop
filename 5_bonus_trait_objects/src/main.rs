use std::fmt::Write;

fn main() {
    let mut window = Window::new("Rust GUI demo");
    // window.add_widget(Box::new(Label::new("This is a small label in the GUI.")));
    // window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}

trait Widget {
    fn width(&self) -> usize;

    fn draw_into(&self, buffer: &mut dyn Write);

    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

struct Window {
    title: String,
}

impl Window {
    fn new(title: &str) -> Self {
        todo!()
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        todo!()
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        todo!()
    }

    fn draw_into(&self, buffer: &mut dyn Write) {
        todo!()
    }
}

struct Label {
    title: String,
}

struct Button {
    label: Label,
}
