use day2::simple_gui::*;
use day2::expression_evaluation::*;
use day2::generic_logger::*;


fn main() {

    // simple_gui
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();

    // expression_evaluation
    test_value();
    test_sum();
    test_recursion();
    test_error();

    // generic_logger
    let l = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    do_things(&l);
}
