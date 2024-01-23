use day1::fibonacci::fib;
use day1::collatz_sequence::collatz_length;
use day1::nested_arrays::transpose;
use day1::geometry::{magnitude, normalize};
use day1::elevator_events::*;
use day1::book_library::*;


fn main() {
    // fibonacci
    let n = 20;
    println!("The {}th fibonacci number is {}", n, fib(n));

    // collatz sequence
    let n = 3;
    println!("The collatz sequence length of {} is {}", n, collatz_length(n));

    // nested arrays
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("The matrix is {:?}, after transposition is {:?}", matrix, transpose(matrix));

    // geometry
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

    // elevator events
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));

    // book library
    let mut library = Library::new();
    println!("Our library is empty: {}", library.is_empty());
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    library.print_books();
    match library.oldest_book() {
       Some(book) => println!("My oldest book is {book}"),
       None => println!("My library is empty!"),
    }
    println!("Our library has {} books", library.len());
}
