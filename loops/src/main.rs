fn main() {
    let mut counter = 0;
    let mut i = 0;

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

    //infinite loop
    //loop {
    //    println!("Hello, world!");
    //}
}
