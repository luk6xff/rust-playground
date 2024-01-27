use day2::simple_gui::*;
use day2::expression_evaluation::*;
use day2::generic_logger::*;
use day2::generic_data_types::*;
use day2::trait_bounds::*;


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

    // generic_data_types
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mut p = Point2 { x: 5, y: 10.0};
    println!("{integer:?} and {float:?} and {p:?}");
    println!("integer_coords: {:?}", integer.coords());
    println!("float_coords: {:?}", float.coords());
    println!("p_coords: {:?}", p.coords());
    p.set_coords(99, 9999 as f64);
    println!("p_coords: {:?}", p.coords());

    // trait_bounds
    let foo = String::from("foo");
    let foo2 = duplicate(foo);
    println!("foo2: {:?}", foo2)
}
