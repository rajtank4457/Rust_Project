use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Constants
const PI: f64 = 3.14159;

fn main() {
    println!("--- Welcome to the Full Rust Demo ---");

    // Section 1: Ownership and Borrowing
    ownership_and_borrowing();

    // Section 2: Generics and Traits
    generics_and_traits();

    // Section 3: Enums and Pattern Matching
    enums_and_pattern_matching();

    // Section 4: Error Handling
    error_handling();

    // Section 5: Iterators and Closures
    iterators_and_closures();

    // Section 6: Async Programming
    async_runtime_demo();

    // Section 7: Multithreading with Mutex
    multithreading_with_mutex();

    // Section 8: Smart Pointers
    smart_pointers_demo();

    // Section 9: Collections
    collections_demo();

    // Section 10: Macros
    macros_demo();

    // Section 11: Command-Line Arguments
    command_line_demo();
}

// Section 1: Ownership and Borrowing
fn ownership_and_borrowing() {
    println!("\n--- Ownership and Borrowing ---");
    let owned_string = String::from("I am owned!");
    let length = calculate_length(&owned_string);
    println!("Length of '{}' is {}", owned_string, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Section 2: Generics and Traits
fn generics_and_traits() {
    println!("\n--- Generics and Traits ---");

    let point = Point { x: 10, y: 20 };
    println!("Point coordinates: ({}, {})", point.x, point.y);

    let circle = Circle { radius: 5.0 };
    println!("Circle area: {:.2}", circle.area());
}

struct Point<T> {
    x: T,
    y: T,
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// Section 3: Enums and Pattern Matching
fn enums_and_pattern_matching() {
    println!("\n--- Enums and Pattern Matching ---");

    let message = Message::Hello(String::from("Rust"));
    match message {
        Message::Hello(msg) => println!("Received message: {}", msg),
        Message::Quit => println!("Quitting"),
    }
}

enum Message {
    Hello(String),
    Quit,
}

// Section 4: Error Handling
fn error_handling() {
    println!("\n--- Error Handling ---");

    let filepath = "nonexistent_file.txt";
    match std::fs::read_to_string(filepath) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}

// Section 5: Iterators and Closures
fn iterators_and_closures() {
    println!("\n--- Iterators and Closures ---");

    let numbers = vec![1, 2, 3, 4];
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    let even_numbers: Vec<_> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
}

// Section 6: Async Programming
fn async_runtime_demo() {
    println!("\n--- Async Programming ---");

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        let handle = tokio::spawn(async_task());
        handle.await.unwrap();
    });
}

async fn async_task() {
    println!("Async task started...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Async task finished!");
}

// Section 7: Multithreading with Mutex
fn multithreading_with_mutex() {
    println!("\n--- Multithreading with Mutex ---");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter value: {}", *counter.lock().unwrap());
}

// Section 8: Smart Pointers
fn smart_pointers_demo() {
    println!("\n--- Smart Pointers ---");

    let boxed_value = Box::new(42);
    println!("Boxed value: {}", boxed_value);

    let rc_value = std::rc::Rc::new(String::from("Shared"));
    println!("RC value: {}", rc_value);
}

// Section 9: Collections
fn collections_demo() {
    println!("\n--- Collections ---");

    let mut hashmap = HashMap::new();
    hashmap.insert("Key1", 100);
    hashmap.insert("Key2", 200);

    for (key, value) in &hashmap {
        println!("{}: {}", key, value);
    }
}

// Section 10: Macros
macro_rules! custom_macro {
    ($msg:expr) => {
        println!("Custom macro says: {}", $msg);
    };
}

fn macros_demo() {
    println!("\n--- Macros ---");
    custom_macro!("Hello from a macro!");
}

// Section 11: Command-Line Arguments
fn command_line_demo() {
    println!("\n--- Command-Line Arguments ---");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Arguments: {:?}", &args[1..]);
    } else {
        println!("No arguments provided.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_length() {
        let test_string = String::from("Rust");
        assert_eq!(calculate_length(&test_string), 4);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 2.0 };
        assert_eq!(circle.area(), 12.56636);
    }
}
