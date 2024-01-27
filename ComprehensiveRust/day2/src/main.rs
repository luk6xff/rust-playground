use day2::simple_gui::*;
use day2::expression_evaluation::*;
use day2::generic_logger::*;
use day2::generic_data_types::*;
use day2::trait_bounds::*;
use day2::impl_trait::*;
use day2::generic_min::*;
use day2::counter::*;
use day2::closures::*;


fn main() {

    println!("\n>>> simple_gui <<<");
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();


    println!("\n>>> expression_evaluation <<<");
    test_value();
    test_sum();
    test_recursion();
    test_error();


    println!("\n>>> generic_logger <<<");
    let l = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    do_things(&l);


    println!("\n>>> generic_data_types <<<");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mut p = Point2 { x: 5, y: 10.0};
    println!("{integer:?} and {float:?} and {p:?}");
    println!("integer_coords: {:?}", integer.coords());
    println!("float_coords: {:?}", float.coords());
    println!("p_coords: {:?}", p.coords());
    p.set_coords(99, 9999 as f64);
    println!("p_coords: {:?}", p.coords());


    println!("\n>>> trait_bounds <<<");
    let foo = String::from("foo");
    let foo2 = duplicate(foo);
    println!("foo2: {:?}", foo2);
    //#[derive(Clone)]
    //struct NonClonable;
    //let non_clonable = NonClonable;
    //let bar = duplicate(non_clonable);
    let foo3 = duplicate(foo2);
    println!("foo3: {:?}", foo3);


    println!("\n>>> impl_trait <<<");
    let many = add_42_millions(11_i16);
    //let many = add_42_millions(11_f64); // this does  not compile
    println!("many: {many:?}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");


    println!("\n>>> generic_min <<<");
    let cit1 = Citation { author: "Shapiro", year: 2011 };
    let cit2 = Citation { author: "Baumann", year: 2010 };
    let cit3 = Citation { author: "Baumann", year: 2019 };
    debug_assert_eq!(min(cit1, cit2), cit2);
    debug_assert_eq!(min(cit2, cit3), cit2);
    debug_assert_eq!(min(cit1, cit3), cit3);
    println!("min(cit1, cit2): {:?}", min(cit1, cit2));
    println!("min(cit2, cit3): {:?}", min(cit2, cit3));
    println!("min(cit1, cit3): {:?}", min(cit1, cit3));


    println!("\n>>> counter <<<");
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));


    println!("\n>>> closures <<<");
    let add_3 = |x| x + 3;
    println!("add_3: {}", apply_with_log(add_3, 5));

    let mut v = Vec::new();
    let mut accumulate = |x| {
        v.push(x);
        v.iter().sum()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 1));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 2));

    let decorate = move |x| {
        println!("decorating {}", x);
        x
    };
    println!("decorate: {}", apply_with_log(decorate, 8));

}
