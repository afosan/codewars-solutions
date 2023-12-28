//! https://www.codewars.com/kata/5a805d8cafa10f8b930005ba/train/rust

pub fn nearest_sq(n: u32) -> u32 {
    let m = (n as f64).sqrt().floor() as u32;
    let s0 = m.pow(2);
    let s1 = (m + 1).pow(2);

    if s1 - n > n - s0 {
        s0
    } else {
        s1
    }
}

#[cfg(test)]
mod tests {
    use super::nearest_sq;

    #[test]
    fn sample_tests() {
        // assertion(expected, n)
        assertion(1, 1);
        assertion(1, 2);
        assertion(9, 10);
        assertion(121, 111);
        assertion(10000, 9999);
    }
    
    fn assertion(expected : u32, n : u32) {
        let actual = nearest_sq(n);
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n n: {}\n", expected, actual, n
        );
    }
}
