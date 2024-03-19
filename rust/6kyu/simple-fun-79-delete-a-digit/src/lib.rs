//! https://www.codewars.com/kata/5894318275f2c75695000146/train/rust

fn helper(n: u32, d: u32) -> u32 {
    let p = 10_u32.pow(d);
    n / p / 10 * p + n % p
}

pub fn delete_digit(n: u32) -> u32 {
    let l = n.to_string().len() as u32;
    (0..l).map(|i| helper(n, i)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(delete_digit(152), 52);
        assert_eq!(delete_digit(1001), 101);
        assert_eq!(delete_digit(10), 1);
    }
}
