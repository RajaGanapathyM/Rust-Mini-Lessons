mod gui;
use crate::gui::{Screen, Button, Checkbox, Label};

fn main() {
    let mut my_screen = Screen::new();

    let button = Box::new(Button {
        label: String::from("Submit"),
    });

    let checkbox = Box::new(Checkbox { checked: true });

    let label = Box::new(Label {
        text: String::from("Welcome to the GUI"),
    });

    my_screen.add_component(button);
    my_screen.add_component(checkbox);
    my_screen.add_component(label);

    my_screen.run();
}
