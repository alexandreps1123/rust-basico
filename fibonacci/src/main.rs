fn main() {
    let n = 10;
    let mut i = 0;
    let mut n1 : i64 = 0;
    let mut n2 : i64 = 1;
    let mut result : i64;

    while i <= n {
        if i == 0 {
            result = n1;
        } else if i == 1 {
            result = n2;
        } else {
            result = n1 + n2;
            n1 = n2;
            n2 = result;
        }

        println!("posicao {i}: {result}");

        i += 1;
    }

    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
}
