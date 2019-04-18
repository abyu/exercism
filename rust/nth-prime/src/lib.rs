pub fn nth(n: u32) -> u32 {
    let range = 2..;
    let prime_numbers = range.filter(|&x| is_prime(x));

    for (i, num) in prime_numbers.enumerate() {
        if i == n as usize {
            return num;
        }
    }

    0
}

fn is_prime(n: u32) -> bool {
    let range = 2..n;
    let mut divisible_by = range.filter(|&x| n % x == 0).peekable();

    divisible_by.peek().is_none()
}
