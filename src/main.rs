// This looks familiar...
use std::io;

// I missed compiled languages so bad.
// Define that main function, baby!
fn main() {

    println!("Guess the number!");

    println!("Please input your guess.");

    // Must define as mutable,
    // also hard-cast as empty?
    let mut guess  = String::new();

    // Use the aforementioned deep sea texts
    io::stdin()
        .read_line(&mut guess)
        // Always use error hadling.
        // Under the sea safety 101.
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
