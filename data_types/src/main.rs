fn main() {
    let _x = 2.0;    // default f64

    let _y: f32 = 2.0;

    // operations
        // addition
        let _sum = 5 + 10;

        // subtraction
        let _difference = 95.5 - 4.3;

        // multiplication
        let _product = 4 * 30;

        // division
        let _quotient = 56.6 / 32.2;
        let _floored = 2 / 3; // 0

        // remainder
        let _remainder = 43 % 5;

    // boolean
    let _t = true;
    let _f: bool = false;

    // char
    let _c = 'z';
    let _z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    
    println!("{heart_eyed_cat}");

    // compound types
        // tuplas
        let tup: (i32, f64, u8) = (500, 6.9, 10);

        // destructuring: get individual values of a tuple
        let (x, _y, _z) = tup;
        println!("{x}");

        let second = tup.1;
        println!("{second}");

        // array
        // fixed length
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        let array_with_three = [2; 5];

        let mut i = 0;
        loop {
            println!("{}", array[i]);
            println!("{}", array_with_three[i]);
            i += 1;
            if i >= array.len() {break};
        }
}
