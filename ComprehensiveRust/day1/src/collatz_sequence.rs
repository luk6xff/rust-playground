pub fn collatz_length(mut n: i32) -> i32 {
    let mut len = 1;
    while n > 1{
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;

        }
        len += 1;
    }

    len
}