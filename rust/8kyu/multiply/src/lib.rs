//! https://www.codewars.com/kata/50654ddff44f800200000004/train/rust

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(multiply(3, 5), 15)
    }
}
