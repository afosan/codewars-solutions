//! https://www.codewars.com/kata/5913152be0b295cf99000001/train/rust

pub fn divisions(n: u32, divisor: u32) -> u32 {
    let mut n = n;
    let mut cnt = 0;
    
    while n >= divisor {
        n /= divisor;
        cnt += 1;
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::divisions;
    
    fn dotest(n: u32, div: u32, expected: u32) {
        let actual = divisions(n, div);
        assert!(actual == expected, "With n = {n}, divisor = {div}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        dotest(6, 2, 2);
        dotest(100, 2, 6);
        dotest(2450, 5, 4);
        dotest(9999, 3, 8);
        dotest(2, 3, 0);
        dotest(5, 5, 1);
    }
}
