
pub struct Library {
    books: Vec<Book>,
}

pub struct Book {
    title: String,
    year: u16,
}

impl Book {
    pub fn new(title:&str, year:u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

pub trait LibraryMethods {
    fn new() -> Library;
    fn is_empty(&self) -> bool;
    fn add_book(&mut self, book: Book);
    fn print_books(&self);
    fn oldest_book(&self) -> Option<&Book>;
    fn len(&self) -> usize;
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A book: {} ({})", self.title, self.year)
    }
}

impl LibraryMethods for Library {

    fn new() -> Library {
        Library {
            books: Vec::<Book>::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.books.len() == 0
    }

    fn add_book(&mut self, book:Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for b in self.books.iter() {
            println!("{}", b);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        match self.is_empty() {
            true => None,
            false => Some(&self.books[0]),}
    }

    fn len(&self) -> usize {
        self.books.len()
    }
}