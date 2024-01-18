//! https://www.codewars.com/kata/5959ec605595565f5c00002b/train/rust

pub fn reverse_bits(n: u32) -> u32 {
    let r = format!("{n:b}").chars().rev().collect::<String>();
    u32::from_str_radix(&r, 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::reverse_bits;
        
    fn do_test(n: u32, expected: u32) {
        let actual = reverse_bits(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        do_test(u32::MAX, u32::MAX);
        do_test(u32::MAX - 1, i32::MAX as u32);
        do_test(417, 267);
        do_test(267, 417);
        do_test(0, 0);
        do_test(2017, 1087);
        do_test(1023, 1023);
        do_test(1024, 1);
    }
}
