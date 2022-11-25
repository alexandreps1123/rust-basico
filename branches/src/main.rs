fn main() {
    let number = 3;

    if number < 5 {
        println!("less than 5");
    } else if number > 5 && number < 10 {
        println!("between 5 and 10");
    } else {
        println!("greater or equals 10");
    }
}
