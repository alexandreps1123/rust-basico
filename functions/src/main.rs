fn main() {
    let x = six();
    let y = increment_one(9);

    println!("Hello, world!");

    println!("six() return -> {x}");

    println!("increment_one(9) return -> {y}");

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

fn six() -> i32 {
    6
}

fn increment_one(x: i32) -> i32 {
    x + 1
}
