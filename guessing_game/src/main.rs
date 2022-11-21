//prelude
use rand::Rng;
use std::cmp::Ordering;
use std::io;

//'fn' new function
fn main() {
    //'println!' is a macro to print a string
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        //'let' create a variable, variables are immutable by default
        //'mut' make a variable mutable
        //'=' bind a value to the variable
        //'String::new' function that returns a new instance of a string
        //'::' indicates that 'new' is an associated function of the 'String' type
        //'new' create a new, empty string
        let mut guess = String::new();

        //'io::stdin' handle with user input
        //'read_line' get user input
        //'&' indicates reference
        //'&mut guess' make reference mutable, to get user input to change the strin's content
        //'read_line' returns a 'Result' value. 'Result' is a enumeration (enum)
        //'expect'  if 'Result' is an 'Err' value, 'expect' will crash the program and display the message
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {guess}");

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
