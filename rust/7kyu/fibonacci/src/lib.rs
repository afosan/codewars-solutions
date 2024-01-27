//! https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af/train/rust

pub fn fib(n: u32) -> u32{
    let (mut n0, mut n1) = (0, 1);

    for _ in 0..n {
        (n0, n1) = (n1, n0 + n1);
    }

    n0
}

#[cfg(test)]
mod tests {
    use super::fib;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(n: u32, expected: u32) {
        assert_eq!(fib(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn returns_expected() {
        dotest(0, 0);
        dotest(1, 1);
        dotest(2, 1);
        dotest(3, 2);
        dotest(4, 3);
        dotest(5, 5);
    }
}
