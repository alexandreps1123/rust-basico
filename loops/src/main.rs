fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 11;
        }
    };

    println!("The result is {result}");
    //infinite loop
    //loop {
    //    println!("Hello, world!");
    //}
}
