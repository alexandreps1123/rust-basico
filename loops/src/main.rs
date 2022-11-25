fn main() {
    let mut counter = 0;
    let mut i = 0;
    let a = [10, 20, 30, 40, 50];

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 11;
        }
    };

    println!("The result is : {result}");

    'counting_up: loop {
        println!("i = {i}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if i == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        i += 1;
    }

    println!("i = {i}");

    // while loop
    while i != 0 {
        println!("{i}!");

        i -= 1;
    }

    //while loop in an array
    while i < 5 {
        println!("value: {}", a[i]);

        i += 1;
    }

    //for loop
    for element in a {
        println!("for element: {element}");
    }

    //reverse the range
    for number in (1..5).rev() {
        println!("{number}");
    }


    //infinite loop
    //loop {
    //    println!("Hello, world!");
    //}
}
