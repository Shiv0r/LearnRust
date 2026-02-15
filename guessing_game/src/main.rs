use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // creates new string

    io::stdin()
        .read_line(&mut guess) // appending string to variable guess
        .expect("Failed to readline"); // catch error

    println!("You guessed: {guess}"); // prints guess as a literal
}
