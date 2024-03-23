//! https://www.codewars.com/kata/56968ce7753513604b000055/train/rust

pub fn pyramid_height(n: u32) -> u32 {
    (1..).take_while(|i| i * (i + 1) * (2 * i + 1) <= 6 * n).last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::pyramid_height;
    
    fn dotest(n: u32, expected: u32) {
        let actual = pyramid_height(n);
        assert!(actual == expected, "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, 1);
        dotest(4, 1);
        dotest(5, 2);
        dotest(29, 3);
        dotest(30, 4);
        dotest(31, 4);
        dotest(1240, 15);
    }
}
