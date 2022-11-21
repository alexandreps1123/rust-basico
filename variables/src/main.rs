fn main() {
    //const cannot recieve a value computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 5;

    let y = 5;

    let y = y + 1;
    println!("The value of x is: {x}");

    println!("The const value: {THREE_HOURS_IN_SECONDS}");
    {
        let y = y * 2;
        println!("The value of y is: {y}");
    }
    x = 6;
    println!("The value of x is: {x}");

    println!("The value of y is: {y}");
}
