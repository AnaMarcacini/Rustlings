use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    let secret_number = 1;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// Returning to the guessing game program, you now know that let mut guess will introduce a mutable variable named guess. The equal sign (=) tells Rust we want to bind something to the variable now. On the right of the equal sign is the value that guess is bound to, which is the result of calling String::new, a function that returns a new instance of a String. String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

// The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.

// In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!