//! https://www.codewars.com/kata/58845748bd5733f1b300001f/train/rust

pub fn range_bit_count(a: u32, b: u32) -> u32 {
    (a..=b).map(|n| format!("{n:b}").chars().filter(|c| *c == '1').count()).sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::range_bit_count;
        
    fn dotest(a: u32, b: u32, expected: u32) {
        let actual = range_bit_count(a, b);
        assert!(actual == expected, "With a = {a}, b = {b}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(2, 7, 11);
        dotest(0, 1, 1);
        dotest(4, 4, 1);
    }
}
