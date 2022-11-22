fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(5);

    multiples_paramenters(10, 'h');
}

fn another_function() {
    println!("Another function!");
}

fn another_function_with_parameters(x: i32) {
    println!("Value parameter {x}");
}

fn multiples_paramenters(x: i32, letter: char) {
    println!("Value parameter {x}{letter}");
}