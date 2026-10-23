// ==========================================
// Task 4 — Vectors and slices
// ==========================================

fn double_in_place(values: &mut [i32]) {
    for val in values.iter_mut() {
        *val *= 2;
    }
}

fn largest(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

fn remove_duplicates(values: &mut Vec<i32>) {
    let mut i = 0;
    while i < values.len() {
        let mut j = i + 1;
        while j < values.len() {
            if values[i] == values[j] {
                values.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

fn running_sum(values: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    values
        .iter()
        .map(|&x| {
            sum += x;
            sum
        })
        .collect()
}

pub fn task4_vectors() {
    let mut v = vec![3, 1, 3, 2, 1];
    remove_duplicates(&mut v);
    double_in_place(&mut v);

    println!("{:?}", v);
    println!("{:?}", running_sum(&v));
    println!("{:?}", largest(&v));
}

// ==========================================
// Task 5 — Structs and methods
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.top_left.x
            && p.x <= self.top_left.x + self.width
            && p.y >= self.top_left.y
            && p.y <= self.top_left.y + self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    pub fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

// ==========================================
// Task 6 — A library
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub year: u16,
}

impl Book {
    pub fn new(title: &str, author: &str, year: u16) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            year,
        }
    }
}

#[derive(Debug, Default)]
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn new() -> Self {
        Library { books: Vec::new() }
    }

    pub fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn titles(&self) -> Vec<&str> {
        self.books.iter().map(|b| b.title.as_str()).collect()
    }

    pub fn by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    pub fn oldest(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|b| b.year)
    }

    pub fn remove(&mut self, title: &str) -> Option<Book> {
        if let Some(pos) = self.books.iter().position(|b| b.title == title) {
            Some(self.books.remove(pos))
        } else {
            None
        }
    }
}

pub fn task6_library() {
    let mut lib = Library::new();

    lib.add(Book::new("The Hobbit", "J.R.R. Tolkien", 1937));
    lib.add(Book::new("The Fellowship of the Ring", "J.R.R. Tolkien", 1954));
    lib.add(Book::new("1984", "George Orwell", 1949));

    println!("Titles: {:?}", lib.titles());
    println!("By Tolkien: {:?}", lib.by_author("J.R.R. Tolkien"));
    println!("Oldest: {:?}", lib.oldest());

    let removed = lib.remove("1984");
    println!("Removed: {:?}", removed);
    println!("Remaining titles: {:?}", lib.titles());
}

fn main() {
    println!("--- Task 4 ---");
    task4_vectors();

    println!("\n--- Task 6 ---");
    task6_library();
}