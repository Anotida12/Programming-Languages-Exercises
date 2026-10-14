// ============================================================================
// PART 1: Fix the Borrow Checker
// ============================================================================

// (a) E0382: borrow of moved value
// Problem: `print_len` originally took ownership of `s`, moving it and making it unavailable for `println!`.
// Fix: Change the function signature to borrow a string slice (`&str`) instead of taking ownership.
fn print_len(s: &str) {
    println!("{}", s.len());
}

fn fix_a() {
    let s = String::from("hello");
    print_len(&s);
    println!("{s}");
}

// (b) E0502: cannot borrow as mutable because it is also borrowed as immutable
// Problem: `first` retained an immutable reference into the vector while `push` required a mutable borrow (&mut),
// which could cause memory reallocation and invalidate `first`.
// Fix: Because `i32` implements `Copy`, copy the value directly rather than holding a reference.
fn fix_b() {
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(4);
    println!("first = {first}");
}

// (f) E0505: cannot move out of `name` because it is borrowed
// Problem: `shout` took `String` by value, moving `name` while the immutable borrow `r` was still active.
// Fix: Change `shout` to accept `&str` instead of taking ownership of `String`.
fn shout(s: &str) -> String {
    s.to_uppercase()
}

fn fix_f() {
    let name = String::from("ada");
    let r = &name;
    let loud = shout(&name);
    println!("{r} {loud}");
}

// (g) E0502: cannot borrow `scores` as mutable because it is also borrowed as immutable
// Problem: The `for s in &scores` loop holds an immutable borrow over `scores`, which prevents calling `scores.push(...)`.
// Fix: Collect matching values into a temporary buffer vector, then extend the original vector after iteration finishes.
fn fix_g() {
    let mut scores = vec![10, 20, 30];
    let mut to_add = Vec::new();

    for s in &scores {
        if *s > 15 {
            to_add.push(*s);
        }
    }
    scores.extend(to_add);
    println!("{scores:?}");
}

// (h) E0499: cannot borrow `c` as mutable more than once at a time
// Problem: Rust prohibits having two simultaneous mutable references (`&mut`) pointing to the same instance.
// Fix: Call `bump()` directly on `c` sequentially rather than creating multiple simultaneous `&mut` bindings.
struct Counter {
    value: i32,
}

impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}

fn fix_h() {
    let mut c = Counter { value: 0 };
    let first_val = c.bump();
    let second_val = c.bump();
    println!("{} {}", first_val, second_val);
}

// (i) E0515: cannot return value referencing local variable
// Problem: The local struct `w` is dropped when `broken_unwrap` exits, so returning a reference into it is invalid.
// Fix: Return an owned `String` so ownership is transferred out to the caller.
struct Wrapper {
    value: String,
}

fn broken_unwrap() -> String {
    let w = Wrapper {
        value: String::from("hi"),
    };
    w.value
}

// ============================================================================
// PART 2: Strings and Slices
// ============================================================================

// 1. Returns text up to the first space, or the whole string.
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}

// 2. Counts vowels (a, e, i, o, u), case-insensitive.
fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

// 3. Reverses word order using split_whitespace.
fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

// 4. Capitalizes the first character of the string.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// 5. Palindrome check ignoring non-alphanumeric characters and case.
fn is_palindrome(s: &str) -> bool {
    let filtered: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    filtered.iter().eq(filtered.iter().rev())
}

// ============================================================================
// Verification / Main Runner
// ============================================================================
fn main() {
    // Run Part 1 fixes
    fix_a();
    fix_b();
    fix_f();
    fix_g();
    fix_h();
    println!("{}", broken_unwrap());

    // Run Part 2 assertions
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("single"), "single");

    assert_eq!(count_vowels("Rust Programming"), 4);

    assert_eq!(reverse_words("the quick brown fox"), "fox brown quick the");

    assert_eq!(capitalize("rust"), "Rust");
    assert_eq!(capitalize(""), "");

    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(!is_palindrome("Hello world"));

    println!("All checks passed!");
}