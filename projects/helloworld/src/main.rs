use std::io;

fn main() {
    println!("Hello, world!");
    println!("Guess the number");
    let mut guess = String::new();

    /* Write a separate function for this */
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Your guess: {guess}");
}
