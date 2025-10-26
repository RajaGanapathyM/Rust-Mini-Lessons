pub trait Widget {
    fn draw(&self);
}

pub struct Button {
    pub label: String,
}

impl Widget for Button {
    fn draw(&self) {
        println!("Drawing a button with label: {}", self.label);
    }
}

pub struct Checkbox {
    pub checked: bool,
}

impl Widget for Checkbox {
    fn draw(&self) {
        println!(
            "Drawing a checkbox which is {}",
            if self.checked { "checked" } else { "unchecked" }
        );
    }
}

pub struct Label {
    pub text: String,
}

impl Widget for Label {
    fn draw(&self) {
        println!("Drawing a label with text: {}", self.text);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Widget>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Widget>) {
        self.components.push(component);
    }

    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}
